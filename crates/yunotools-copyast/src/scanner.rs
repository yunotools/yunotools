use crate::IncrementalTracker;
use crate::binary::detect_binary;
use crate::config::CopyastConfig;
use crate::domain::{ScanStats, TextFile};
use crate::ignore::IgnoreEngine;
use crate::writer::Writer;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::{fs, io};

// Kết quả nội bộ của quá trình scan.
pub struct ScanOutput {
    pub files: Vec<TextFile>,
    pub stats: ScanStats,
}

//Nhận input
//    ↓
// Input là file?
//    ├── Có → kiểm tra trực tiếp file đó
//    └── Không → tạo WalkBuilder để duyệt folder
//                     ↓
//              Áp dụng ignore
//                     ↓
//              Kiểm tra kích thước
//                     ↓
//              Kiểm tra binary
//                     ↓
//              Đọc nội dung UTF-8
//                     ↓
//              Tạo TextFile
pub struct Scanner;

impl Scanner {
    pub fn scan(root: &Path) -> (Vec<TextFile>, usize) {
        let config = CopyastConfig::new(root, PathBuf::new());

        let output = Self::scan_with_config(&config);
        let skipped = output.stats.skipped();

        (output.files, skipped)
    }

    pub fn scan_with_config(config: &CopyastConfig) -> ScanOutput {
        let mut files = Vec::new();
        let mut stats = ScanStats::default();

        let excluded_paths = if config.output.as_os_str().is_empty() {
            Vec::new()
        } else {
            let cache_path = IncrementalTracker::cache_path_for(&config.output);

            let temporary_cache_path = IncrementalTracker::temporary_cache_path_for(&config.output);

            let temporary_output_path = Writer::temporary_path_for(&config.output);

            vec![
                to_absolute_path(&config.output),
                to_absolute_path(&cache_path),
                to_absolute_path(&temporary_cache_path),
                to_absolute_path(&temporary_output_path),
            ]
        };

        // Nếu client truyền trực tiếp vào 1 file
        if config.input.is_file() {
            Self::collect_file(
                &config.input,
                &excluded_paths,
                config,
                &mut files,
                &mut stats,
            );

            return ScanOutput { files, stats };
        }

        // Else: truyền vào 1 folder
        let mut builder = WalkBuilder::new(&config.input);

        IgnoreEngine::configure_walker(&mut builder, config.respect_ignore, config.include_hidden);

        for entry_result in builder.build() {
            let entry = match entry_result {
                Ok(entry) => entry,

                Err(_) => {
                    stats.unreadable += 1;
                    continue;
                }
            };

            let is_file = entry
                .file_type()
                .map(|file_type| file_type.is_file())
                .unwrap_or(false);

            if !is_file {
                continue;
            }

            Self::collect_file(
                entry.path(),
                &excluded_paths,
                config,
                &mut files,
                &mut stats,
            );
        }

        // Sort để giữ thứ tự output ổn định giữa các lần chạy
        files.sort_by(|left, right| left.path.cmp(&right.path));

        ScanOutput { files, stats }
    }

    // Kiểm tra và đọc 1 file
    fn collect_file(
        path: &Path,
        excluded_paths: &[PathBuf],
        config: &CopyastConfig,
        files: &mut Vec<TextFile>,
        stats: &mut ScanStats,
    ) {
        stats.discovered += 1;

        let is_excluded = excluded_paths
            .iter()
            .any(|excluded_path| same_path(path, excluded_path));

        if is_excluded {
            stats.ignored += 1;
            return;
        }

        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,

            Err(_) => {
                stats.unreadable += 1;
                return;
            }
        };

        if metadata.len() > config.max_file_size {
            stats.too_large += 1;
            return;
        }

        match detect_binary(path) {
            Ok(true) => {
                stats.binary += 1;
                return;
            }

            Ok(false) => {}

            Err(_) => {
                stats.unreadable += 1;
                return;
            }
        }

        let content = match fs::read_to_string(path) {
            Ok(content) => content,

            // InvalidData thường có nghĩa nội dung không phải UTF-8
            Err(error) if error.kind() == io::ErrorKind::InvalidData => {
                stats.binary += 1;
                return;
            }

            Err(_) => {
                stats.unreadable += 1;
                return;
            }
        };

        files.push(TextFile::new(path, content));
        stats.copied += 1;
    }
}

// Chuyển một path thành absolute path để so sánh.
fn to_absolute_path(path: &Path) -> PathBuf {
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }

    if path.is_absolute() {
        return path.to_path_buf();
    }

    match std::env::current_dir() {
        Ok(current_dir) => current_dir.join(path),
        Err(_) => path.to_path_buf(),
    }
}

// So sánh 2 đường dẫn
// Trên Windows, đường dẫn không phân biệt chữ hoa/chữ thường
fn same_path(left: &Path, right: &Path) -> bool {
    let left = to_absolute_path(left);
    let right = to_absolute_path(right);

    if cfg!(windows) {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    } else {
        left == right
    }
}
