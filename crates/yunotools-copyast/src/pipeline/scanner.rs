use crate::config::CopyastConfig;
use crate::domain::{ScanStats, TextFile};
use crate::ignore::configure_walker;
use crate::pipeline::binary::detect_binary;
use crate::pipeline::file_ops::paths_equal;
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
pub(crate) fn scan(config: &CopyastConfig) -> ScanOutput {
    let mut files = Vec::new();
    let mut stats = ScanStats::default();
    let excluded_paths = excluded_paths(config);

    if config.input.is_file() {
        collect_file(
            &config.input,
            &excluded_paths,
            config,
            &mut files,
            &mut stats,
        );

        return ScanOutput { files, stats };
    }

    let mut builder = WalkBuilder::new(&config.input);
    configure_walker(
        &mut builder,
        config.respect_ignore,
        config.include_hidden,
        &config.additional_ignore_files,
    );

    for entry_result in builder.build() {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(_) => {
                stats.unreadable += 1;
                continue;
            }
        };

        if !entry
            .file_type()
            .is_some_and(|file_type| file_type.is_file())
        {
            continue;
        }

        collect_file(
            entry.path(),
            &excluded_paths,
            config,
            &mut files,
            &mut stats,
        );
    }

    // Giữ thứ tự output ổn định giữa các lần chạy.
    files.sort_by(|left, right| left.path.cmp(&right.path));

    ScanOutput { files, stats }
}

fn excluded_paths(config: &CopyastConfig) -> Vec<PathBuf> {
    if config.output.as_os_str().is_empty() {
        return Vec::new();
    }

    vec![
        config.output.clone(),
        IncrementalTracker::cache_path_for(&config.output),
        IncrementalTracker::temporary_cache_path_for(&config.output),
        writer::temporary_path_for(&config.output),
    ]
}

fn collect_file(
    path: &Path,
    excluded_paths: &[PathBuf],
    config: &CopyastConfig,
    files: &mut Vec<TextFile>,
    stats: &mut ScanStats,
) {
    stats.discovered += 1;

    if excluded_paths
        .iter()
        .any(|excluded_path| paths_equal(path, excluded_path))
    {
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
        // InvalidData thường có nghĩa nội dung không phải UTF-8.
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
