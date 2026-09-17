use crate::config::{CopyastConfig, PathMode};
use crate::domain::TextFile;
use crate::pipeline::file_ops::{
    append_suffix, create_parent_directory, replace_file, resolve_absolute_path,
};
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

const COPYAST_BANNER: &str = "=== Yunotools-Copyast ===";

pub(crate) fn write_output(files: &[TextFile], config: &CopyastConfig) -> io::Result<()> {
    create_parent_directory(&config.output_path)?;

    // Ghi ra file tạm trước để output cũ không bị dở dang nếu có lỗi.
    let temporary_output = build_temporary_path(&config.output_path);

    if let Err(error) = write_files_to_path(files, config, &temporary_output) {
        let _ = fs::remove_file(&temporary_output);
        return Err(error);
    }

    if let Err(error) = replace_file(&temporary_output, &config.output_path) {
        let _ = fs::remove_file(&temporary_output);
        return Err(error);
    }

    Ok(())
}

pub(crate) fn build_temporary_path(output_path: &Path) -> PathBuf {
    append_suffix(output_path, ".copyast-tmp")
}

// Dùng chung cho Writer và TokenEstimator để hai module tính cùng một header.
pub(crate) fn render_header(file: &TextFile, config: &CopyastConfig) -> String {
    let displayed_path = resolve_header_path(&file.path, config);

    format!(
        "{COPYAST_BANNER}\n=== FILE: {} ===\n",
        displayed_path.display(),
    )
}

fn write_files_to_path(
    files: &[TextFile],
    config: &CopyastConfig,
    output_path: &Path,
) -> io::Result<()> {
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    for (file_index, file) in files.iter().enumerate() {
        // Chèn một dòng trống giữa hai file.
        if file_index > 0 {
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

fn resolve_header_path(file_path: &Path, config: &CopyastConfig) -> PathBuf {
    let should_use_absolute_path = match config.path_mode {
        PathMode::Absolute => true,
        PathMode::Relative => false,
        PathMode::Auto => config.input_path.is_absolute(),
    };

    if should_use_absolute_path {
        return resolve_absolute_path(file_path);
    }

    let input_root = if config.input_path.is_file() {
        config.input_path.parent().unwrap_or(Path::new("."))
    } else {
        &config.input_path
    };

    if let Ok(relative_path) = file_path.strip_prefix(input_root) {
        return relative_path.to_path_buf();
    }

    let absolute_file_path = resolve_absolute_path(file_path);
    let absolute_input_root = resolve_absolute_path(input_root);

    absolute_file_path
        .strip_prefix(absolute_input_root)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| file_path.to_path_buf())
}
