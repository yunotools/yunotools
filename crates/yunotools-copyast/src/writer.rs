use crate::CopyastConfig;
use crate::config::PathMode;
use crate::domain::TextFile;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

const COPYAST_BANNER: &str = "******************Yunotools-Copyast******************";

pub struct Writer;

impl Writer {
    pub fn write(files: &[TextFile], output: impl AsRef<Path>) -> io::Result<()> {
        Self::write_internal(files, output.as_ref(), None)
    }

    pub fn write_with_config(files: &[TextFile], config: &CopyastConfig) -> io::Result<()> {
        Self::write_internal(files, &config.output, Some(config))
    }

    // Scanner sử dụng function này để không đọc
    // nhầm output đang được ghi dở
    pub(crate) fn temporary_path_for(output: impl AsRef<Path>) -> PathBuf {
        let mut temporary_path = output.as_ref().as_os_str().to_os_string();

        temporary_path.push(".copyast-tmp");

        PathBuf::from(temporary_path)
    }

    fn write_internal(
        files: &[TextFile],
        output: &Path,
        config: Option<&CopyastConfig>,
    ) -> io::Result<()> {
        create_parent_directory(output)?;

        let temporary_output = Self::temporary_path_for(output);

        // File tạm thực hiện:
        // - Tạo file nếu chưa tồn tại.
        // - Xóa nội dung cũ nếu file đã tồn tại.
        // - Mở file ở chế độ ghi.
        let output_file = File::create(&temporary_output)?;

        // Dùng BufWriter
        // Nếu ghi trực tiếp vào File, mỗi lần gọi writeln!() có thể tạo một lần ghi xuống hệ điều hành
        // let mut writer = BufWriter::new(output_file);
        // BufWriter giữ dữ liệu tạm trong RAM và ghi xuống ổ đĩa theo từng khối lớn. Điều này hiệu quả hơn khi ghép nhiều file. Khi hoàn tất:
        // writer.flush()
        // flush() đẩy toàn bộ dữ liệu còn lại trong buffer xuống file.
        let mut writer = BufWriter::new(output_file);

        // enumerate() cung cấp cả vị trí và giá trị:
        // index = 0, text_file = file đầu tiên
        // index = 1, text_file = file thứ hai
        // index = 2, text_file = file thứ ba
        for (index, text_file) in files.iter().enumerate() {
            // Chèn một dòng trống giữa hai file
            // File đầu tiên không cần dòng trống ở phía trên
            if index > 0 {
                writeln!(writer)?;
            }

            let displayed_path = path_for_header(&text_file.path, config);

            writeln!(writer, "{}", COPYAST_BANNER)?;

            writeln!(
                writer,
                "******************{}******************",
                displayed_path.display(),
            )?;

            writer.write_all(text_file.content.as_bytes())?;

            // Bảo đảm file tiếp theo luôn bắt đầu trên một dòng mới
            if !text_file.content.ends_with('\n') {
                writeln!(writer)?;
            }
        }

        writer.flush()?;

        writer.get_ref().sync_all()?;

        // Đóng handle trước khi rename,
        // đặc biệt quan trọng trên Windows
        drop(writer);

        replace_output_file(&temporary_output, output)
    }
}

// Tạo parent directory của output nếu chưa tồn tại
// output = "exports/ai/context.txt"
// Function sẽ tạo folder "exports/ai".
fn create_parent_directory(output: &Path) -> io::Result<()> {
    let Some(parent) = output.parent() else {
        return Ok(());
    };

    if parent.as_os_str().is_empty() {
        return Ok(());
    }

    fs::create_dir_all(parent)
}

// Tính đường dẫn sẽ xuất hiện trong header
fn path_for_header(file_path: &Path, config: Option<&CopyastConfig>) -> PathBuf {
    let Some(config) = config else {
        return file_path.to_path_buf();
    };

    let use_absolute_path = match config.path_mode {
        PathMode::Absolute => true,
        PathMode::Relative => false,

        PathMode::Auto => config.input.is_absolute(),
    };

    if use_absolute_path {
        return to_absolute_path(file_path);
    }

    let input_root = if config.input.is_file() {
        config.input.parent().unwrap_or(Path::new("."))
    } else {
        &config.input
    };

    // Trường hợp file_path và input_root đều là relative
    // hoặc đều là absolute
    if let Ok(relative_path) = file_path.strip_prefix(input_root) {
        return relative_path.to_path_buf();
    }

    // Trường hợp một path relative, path còn lại absolute
    let absolute_file = to_absolute_path(file_path);
    let absolute_root = to_absolute_path(input_root);

    if let Ok(relative_path) = absolute_file.strip_prefix(&absolute_root) {
        return relative_path.to_path_buf();
    }

    // Nếu không thể tính relative path,
    // sử dụng path ban đầu thay vì gây lỗi
    file_path.to_path_buf()
}

// Chuyển relative path thành absolute path
fn to_absolute_path(file_path: &Path) -> PathBuf {
    if file_path.is_absolute() {
        return file_path.to_path_buf();
    }

    match std::env::current_dir() {
        Ok(current_dir) => current_dir.join(file_path),
        Err(_) => file_path.to_path_buf(),
    }
}

// Đưa output tạm thành output chính thức
fn replace_output_file(temporary_path: &Path, output: &Path) -> io::Result<()> {
    // Trên Windows, rename không ghi đè
    // lên file đã tồn tại
    #[cfg(windows)]
    if output.exists() {
        fs::remove_file(output)?;
    }

    fs::rename(temporary_path, output)
}
