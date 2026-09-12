use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// Thêm một suffix vào cuối đường dẫn
// Ví dụ:
// context.txt + ".copyast-tmp"
// → context.txt.copyast-tmp
pub(crate) fn append_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut result = path.as_os_str().to_os_string();

    result.push(suffix);

    PathBuf::from(result)
}

// Tạo thư mục cha nếu chưa tồn tại
// Ví dụ:
// exports/ai/context.txt
// → tạo exports/ai/
pub(crate) fn create_parent_directory(path: &Path) -> io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    if parent.as_os_str().is_empty() {
        return Ok(());
    }

    fs::create_dir_all(parent)
}

// Đưa file tạm thành file chính thức
// Trên Unix, rename có thể thay thế file đích
// Trên Windows, file đích cần được xóa trước
pub(crate) fn replace_file(source: &Path, destination: &Path) -> io::Result<()> {
    #[cfg(windows)]
    if destination.exists() {
        fs::remove_file(destination)?;
    }

    fs::rename(source, destination)
}
