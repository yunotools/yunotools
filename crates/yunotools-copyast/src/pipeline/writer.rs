use crate::config::{CopyastConfig, PathMode};
use crate::domain::TextFile;
use crate::pipeline::file_ops::{
    absolute_path, append_suffix, create_parent_directory, replace_file,
};
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

const COPYAST_BANNER: &str = "******************Yunotools-Copyast******************";

pub(crate) fn write(files: &[TextFile], config: &CopyastConfig) -> io::Result<()> {
    create_parent_directory(&config.output)?;

    // Ghi ra file tạm trước để output cũ không bị dở dang nếu có lỗi.
    let temporary_output = temporary_path_for(&config.output);

    if let Err(error) = write_to_path(files, config, &temporary_output) {
        let _ = fs::remove_file(&temporary_output);
        return Err(error);
    }

    if let Err(error) = replace_file(&temporary_output, &config.output) {
        let _ = fs::remove_file(&temporary_output);
        return Err(error);
    }

    Ok(())
}

pub(crate) fn temporary_path_for(output: &Path) -> PathBuf {
    append_suffix(output, ".copyast-tmp")
}

// Dùng chung cho Writer và TokenEstimator để hai module tính cùng một header.
pub(crate) fn render_header(file: &TextFile, config: &CopyastConfig) -> String {
    let displayed_path = path_for_header(&file.path, config);

    format!(
        "{COPYAST_BANNER}\n******************{}******************\n",
        displayed_path.display(),
    )
}

fn write_to_path(files: &[TextFile], config: &CopyastConfig, output: &Path) -> io::Result<()> {
    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);

    for (index, file) in files.iter().enumerate() {
        // Chèn một dòng trống giữa hai file.
        if index > 0 {
            writeln!(writer)?;
        }

        writer.write_all(render_header(file, config).as_bytes())?;
        writer.write_all(file.content.as_bytes())?;

        // Bảo đảm file tiếp theo luôn bắt đầu trên một dòng mới.
        if !file.content.ends_with('\n') {
            writeln!(writer)?;
        }
    }

    writer.flush()?;
    writer.get_ref().sync_all()
}

fn path_for_header(file_path: &Path, config: &CopyastConfig) -> PathBuf {
    let use_absolute_path = match config.path_mode {
        PathMode::Absolute => true,
        PathMode::Relative => false,
        PathMode::Auto => config.input.is_absolute(),
    };

    if use_absolute_path {
        return absolute_path(file_path);
    }

    let input_root = if config.input.is_file() {
        config.input.parent().unwrap_or(Path::new("."))
    } else {
        &config.input
    };

    if let Ok(relative_path) = file_path.strip_prefix(input_root) {
        return relative_path.to_path_buf();
    }

    let absolute_file = absolute_path(file_path);
    let absolute_root = absolute_path(input_root);

    absolute_file
        .strip_prefix(absolute_root)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| file_path.to_path_buf())
}
