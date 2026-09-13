use crate::config::CopyastConfig;
use crate::domain::{ScanStats, TextFile};
use crate::ignore::configure_walker;
use crate::pipeline::binary::is_binary_file;
use crate::pipeline::file_ops::are_paths_equal;
use crate::pipeline::incremental::IncrementalTracker;
use crate::pipeline::writer;
use ignore::WalkBuilder;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub(crate) struct ScanOutput {
    pub(crate) files: Vec<TextFile>,
    pub(crate) stats: ScanStats,
}

// Nhận input
//    ↓
// Input là file?
//    ├── Có → kiểm tra trực tiếp file đó
//    └── Không → duyệt folder, áp dụng ignore, kiểm tra binary và đọc UTF-8
pub(crate) fn scan_files(config: &CopyastConfig) -> ScanOutput {
    let mut text_files = Vec::new();
    let mut stats = ScanStats::default();
    let excluded_paths = build_excluded_paths(config);

    if config.input_path.is_file() {
        try_collect_file(
            &config.input_path,
            &excluded_paths,
            config,
            &mut text_files,
            &mut stats,
        );

        return ScanOutput {
            files: text_files,
            stats,
        };
    }

    let mut walker_builder = WalkBuilder::new(&config.input_path);
    configure_walker(
        &mut walker_builder,
        config.should_respect_ignore,
        config.should_include_hidden,
        &config.additional_ignore_file_names,
    );

    for entry_result in walker_builder.build() {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(_) => {
                stats.unreadable_files += 1;
                continue;
            }
        };

        if !entry
            .file_type()
            .is_some_and(|file_type| file_type.is_file())
        {
            continue;
        }

        try_collect_file(
            entry.path(),
            &excluded_paths,
            config,
            &mut text_files,
            &mut stats,
        );
    }

    // Giữ thứ tự output ổn định giữa các lần chạy.
    text_files.sort_by(|left, right| left.path.cmp(&right.path));

    ScanOutput {
        files: text_files,
        stats,
    }
}

fn build_excluded_paths(config: &CopyastConfig) -> Vec<PathBuf> {
    if config.output_path.as_os_str().is_empty() {
        return Vec::new();
    }

    vec![
        config.output_path.clone(),
        IncrementalTracker::build_cache_path(&config.output_path),
        IncrementalTracker::build_temporary_cache_path(&config.output_path),
        writer::build_temporary_path(&config.output_path),
    ]
}

fn try_collect_file(
    path: &Path,
    excluded_paths: &[PathBuf],
    config: &CopyastConfig,
    files: &mut Vec<TextFile>,
    stats: &mut ScanStats,
) {
    stats.discovered_files += 1;

    if excluded_paths
        .iter()
        .any(|excluded_path| are_paths_equal(path, excluded_path))
    {
        stats.ignored_files += 1;
        return;
    }

    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => {
            stats.unreadable_files += 1;
            return;
        }
    };

    if metadata.len() > config.max_file_size_bytes {
        stats.oversized_files += 1;
        return;
    }

    match is_binary_file(path) {
        Ok(true) => {
            stats.binary_files += 1;
            return;
        }
        Ok(false) => {}
        Err(_) => {
            stats.unreadable_files += 1;
            return;
        }
    }

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        // InvalidData thường có nghĩa nội dung không phải UTF-8.
        Err(error) if error.kind() == io::ErrorKind::InvalidData => {
            stats.binary_files += 1;
            return;
        }
        Err(_) => {
            stats.unreadable_files += 1;
            return;
        }
    };

    files.push(TextFile::new(path, content));
    stats.copied_files += 1;
}
