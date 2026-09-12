use crate::path::normalize;
use crate::scanner::Scanner;
use crate::writer::Writer;
use crate::{
    CopyastConfig, CopyastError, CopyastResult, DuplicateDetector, FrameworkDetector,
    IncrementalTracker, LanguageDetector, TokenEstimator,
};
use std::path::{Path, PathBuf};
use yunotools_core::logger;

pub struct CopyastService;

impl CopyastService {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = CopyastConfig::new(normalize(input), output);

        self.run_with_config(&config)?;

        Ok(())
    }

    pub fn run_with_config(&self, config: &CopyastConfig) -> Result<CopyastResult, CopyastError> {
        validate_config(config)?;

        logger::info("Copyast scanning started");

        // Bước 1: quét và đọc các file text.
        let scan_output = Scanner::scan_with_config(config);

        let mut files = scan_output.files;
        let scan_stats = scan_output.stats;

        logger::info(&format!("Copied files: {}", scan_stats.copied,));

        logger::warn(&format!("Skipped files: {}", scan_stats.skipped(),));

        // Nếu input là một file, marker ngôn ngữ cần được
        // tìm trong thư mục chứa file đó
        let detection_root = find_detection_root(config);

        // Bước 2: nhận diện ngôn ngữ
        let detected = LanguageDetector::analyze(detection_root, &files);

        let detected_languages = detected
            .into_iter()
            .map(|item| item.language.to_string())
            .collect::<Vec<_>>();

        logger::info(&format!("Detected languages: {:?}", detected_languages,));

        let detected_frameworks = FrameworkDetector::analyze(detection_root, &files)
            .into_iter()
            .map(|item| item.framework.to_string())
            .collect::<Vec<_>>();

        logger::info(&format!("Detected frameworks: {:?}", detected_frameworks,));

        // Bước 3: tìm duplicate trước khi xóa
        // Nhờ vậy kết quả vẫn báo cáo được những file
        // nào trùng nhau, kể cả khi deduplicate được bật
        let duplicate_groups = DuplicateDetector::find(&files);

        if config.deduplicate {
            let removed_count = DuplicateDetector::remove_duplicates(&mut files);

            logger::info(&format!("Removed duplicate files: {removed_count}",));
        }

        // Bước 4: ước lượng token sau khi đã deduplicate
        // Đây là lượng token gần với nội dung thực tế
        // sẽ được ghi ra output
        let token_estimate = TokenEstimator::estimate(&files, config.token_model);

        logger::info(&format!(
            "Estimated tokens for {}: {}",
            token_estimate.model.name(),
            token_estimate.tokens,
        ));

        // Tổng dung lượng nội dung các file sau khi deduplicate
        let total_bytes = files
            .iter()
            .fold(0_u64, |total, file| total.saturating_add(file.size_bytes()));

        // Bước 5: quyết định có ghi output hay không
        let incremental_stats = if config.incremental {
            run_incremental(config, &files)?
        } else {
            run_normal(config, &files)?;
            None
        };

        logger::success("Copyast finished");

        Ok(CopyastResult {
            output: config.output.clone(),
            scan: scan_stats,
            total_bytes,
            estimated_tokens: token_estimate.tokens,
            detected_languages,
            detected_frameworks,
            duplicate_groups,
            incremental: incremental_stats,
        })
    }
}

// Kiểm tra các giá trị bắt buộc trước khi chạy
fn validate_config(config: &CopyastConfig) -> Result<(), CopyastError> {
    if !config.input.exists() {
        return Err(CopyastError::InputNotFound {
            path: config.input.clone(),
        });
    }

    if config.output.as_os_str().is_empty() {
        return Err(CopyastError::EmptyOutputPath);
    }

    if !config.input.is_file() && !config.input.is_dir() {
        return Err(CopyastError::UnsupportedInputType {
            path: config.input.clone(),
        });
    }

    if config.output.is_dir() {
        return Err(CopyastError::OutputIsDirectory {
            path: config.output.clone(),
        });
    }

    // Vì sao phải kiểm tra input và output trùng nhau?
    // Giả sử người dùng chạy:
    //  - input  = src/main.rs
    //  - output = src/main.rs
    // Writer sử dụng:
    // Nếu không ngăn chặn, File::create() sẽ xóa nội dung cũ của src/main.rs trước khi ghi.
    // Đây là tình huống có thể làm mất source code.
    // Bây giờ service sẽ trả lỗi:
    //  - input and output cannot be the same file: src/main.rs
    if config.input.is_file() && paths_are_same(&config.input, &config.output) {
        return Err(CopyastError::InputOutputConflict {
            path: config.input.clone(),
        });
    }

    Ok(())
}

// So sánh hai đường dẫn có cùng trỏ đến một vị trí hay không
fn paths_are_same(left: &Path, right: &Path) -> bool {
    let left = path_for_comparison(left);
    let right = path_for_comparison(right);

    if cfg!(windows) {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    } else {
        left == right
    }
}

// Chuyển đường dẫn sang dạng phù hợp để so sánh.
fn path_for_comparison(path: &Path) -> PathBuf {
    // canonicalize() xử lý:
    // - relative path;
    // - ký hiệu "." và "..";
    // - symbolic link;
    // - đường dẫn thật trên filesystem.
    if let Ok(canonical_path) = path.canonicalize() {
        return canonical_path;
    }

    // Output có thể chưa tồn tại nên canonicalize() có thể thất bại.
    // Nếu nó đã absolute thì giữ nguyên
    if path.is_absolute() {
        return path.to_path_buf();
    }

    // Với relative output chưa tồn tại,
    // ghép nó với current working directory
    match std::env::current_dir() {
        Ok(current_directory) => current_directory.join(path),
        Err(_) => path.to_path_buf(),
    }
}

// Xác định folder dùng để tìm marker ngôn ngữ
// Ví dụ nếu input là:
// src/main.rs
// Ta cần tìm Cargo.toml từ folder:
// src/
fn find_detection_root(config: &CopyastConfig) -> &Path {
    if config.input.is_file() {
        return config.input.parent().unwrap_or_else(|| Path::new("."));
    }

    config.input.as_path()
}

// Chế độ bình thường luôn ghi lại output,
// trừ khi người dùng bật dry-run
fn run_normal(config: &CopyastConfig, files: &[crate::TextFile]) -> Result<(), CopyastError> {
    if config.dry_run {
        logger::info("Dry-run enabled: output was not written");

        return Ok(());
    }

    Writer::write_with_config(files, config)?;

    Ok(())
}

// Chế độ incremental chỉ ghi khi có thay đổi
fn run_incremental(
    config: &CopyastConfig,
    files: &[crate::TextFile],
) -> Result<Option<crate::IncrementalStats>, CopyastError> {
    let tracker = IncrementalTracker::analyze(config, files)?;

    let mut stats = tracker.stats();

    if config.dry_run {
        logger::info("Dry-run enabled: output and cache were not written");

        return Ok(Some(stats));
    }

    if !tracker.needs_update() {
        logger::info("No source changes detected: output was not rewritten");

        return Ok(Some(stats));
    }

    // Phải ghi output thành công trước
    Writer::write_with_config(files, config)?;

    // Chỉ lưu cache sau khi output đã ghi thành công
    // Nếu Writer gặp lỗi, cache cũ vẫn được giữ nguyên
    // nhờ vậy lần chạy sau không hiểu nhầm rằng output đã được cập nhật
    tracker.save()?;

    stats.output_written = true;

    Ok(Some(stats))
}
