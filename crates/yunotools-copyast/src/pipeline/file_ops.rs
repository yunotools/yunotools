//! Các thao tác đường dẫn và ghi file dùng chung trong pipeline.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Thêm suffix vào cuối đường dẫn mà không làm mất extension.
// Ví dụ: context.txt + ".copyast-tmp" -> context.txt.copyast-tmp.
pub(crate) fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut result = path.as_os_str().to_os_string();
    result.push(suffix);
    PathBuf::from(result)
}

// canonicalize() xử lý relative path, ".", ".." và symbolic link.
// Khi output chưa tồn tại, ta ghép relative path với current directory.
pub(crate) fn absolute_path(path: &Path) -> PathBuf {
    if let Ok(canonical_path) = path.canonicalize() {
        return canonical_path;
    }

    if path.is_absolute() {
        return path.to_path_buf();
    }

    std::env::current_dir()
        .map(|current_directory| current_directory.join(path))
        .unwrap_or_else(|_| path.to_path_buf())
}

// Trên Windows, đường dẫn thường không phân biệt chữ hoa/chữ thường.
pub(crate) fn paths_equal(left: &Path, right: &Path) -> bool {
    let left = absolute_path(left);
    let right = absolute_path(right);

    if cfg!(windows) {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    } else {
        left == right
    }
}

// Ví dụ: exports/ai/context.txt sẽ tạo folder exports/ai nếu cần.
pub(crate) fn create_parent_directory(path: &Path) -> io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    if parent.as_os_str().is_empty() {
        return Ok(());
    }

    fs::create_dir_all(parent)
}

// Trên Unix, rename có thể thay thế file đích.
// Trên Windows, file đích cần được xóa trước.
pub(crate) fn replace_file(source: &Path, destination: &Path) -> io::Result<()> {
    #[cfg(windows)]
    if destination.exists() {
        fs::remove_file(destination)?;
    }

    fs::rename(source, destination)
}
