use crate::analysis::{DuplicateDetector, FrameworkDetector, LanguageDetector, TokenEstimator};
use crate::config::CopyastConfig;
use crate::domain::{CopyastResult, IncrementalStats, TextFile};
use crate::error::CopyastError;
use crate::pipeline::file_ops::paths_equal;
use crate::pipeline::incremental::IncrementalTracker;
use crate::pipeline::{scanner, writer};
use std::path::Path;
use yunotools_core::logger;

// Facade điều phối toàn bộ pipeline Copyast.
#[derive(Default)]
pub struct CopyastService;

impl CopyastService {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, config: &CopyastConfig) -> Result<CopyastResult, CopyastError> {
        validate_config(config)?;
        logger::info("Copyast scanning started");

        let scan_output = scanner::scan(config);
        let mut files = scan_output.files;
        let scan_stats = scan_output.stats;

        logger::info(&format!("Copied files: {}", scan_stats.copied));

        if scan_stats.skipped() > 0 {
            logger::warn(&format!("Skipped files: {}", scan_stats.skipped()));
        }

        let detection_root = detection_root(config);

        let detected_languages = LanguageDetector::analyze(detection_root, &files)
            .into_iter()
            .map(|detected| detected.language.to_string())
            .collect::<Vec<_>>();

        logger::info(&format!("Detected languages: {detected_languages:?}"));

        let detected_frameworks = FrameworkDetector::analyze(detection_root, &files)
            .into_iter()
            .map(|detected| detected.framework.to_string())
            .collect::<Vec<_>>();

        logger::info(&format!("Detected frameworks: {detected_frameworks:?}"));

        // Tìm nhóm trùng trước khi xóa để vẫn trả đủ báo cáo.
        let duplicate_groups = DuplicateDetector::find_groups(&files);

        if config.deduplicate {
            let removed_count = DuplicateDetector::remove_duplicates(&mut files);
            logger::info(&format!("Removed duplicate files: {removed_count}"));
        }

        let token_estimate = TokenEstimator::estimate(&files, config);

        logger::info(&format!(
            "Estimated tokens for {}: {}",
            token_estimate.model.name(),
            token_estimate.tokens,
        ));

        let incremental = if config.incremental {
            Some(write_incremental_output(config, &files)?)
        } else {
            write_output(config, &files)?;
            None
        };

        logger::success("Copyast finished");

        Ok(CopyastResult {
            output: config.output.clone(),
            scan: scan_stats,
            total_bytes: token_estimate.bytes,
            estimated_tokens: token_estimate.tokens,
            detected_languages,
            detected_frameworks,
            duplicate_groups,
            incremental,
        })
    }
}

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

    // File input và output trùng nhau sẽ làm Writer ghi đè source.
    if config.input.is_file() && paths_equal(&config.input, &config.output) {
        return Err(CopyastError::InputOutputConflict {
            path: config.input.clone(),
        });
    }

    Ok(())
}

fn detection_root(config: &CopyastConfig) -> &Path {
    if config.input.is_file() {
        return config.input.parent().unwrap_or(Path::new("."));
    }

    &config.input
}

fn write_output(config: &CopyastConfig, files: &[TextFile]) -> Result<(), CopyastError> {
    if config.dry_run {
        logger::info("Dry-run enabled: output was not written");
        return Ok(());
    }

    writer::write(files, config)?;
    Ok(())
}

fn write_incremental_output(
    config: &CopyastConfig,
    files: &[TextFile],
) -> Result<IncrementalStats, CopyastError> {
    let tracker = IncrementalTracker::analyze(config, files)?;
    let mut stats = tracker.stats();

    if config.dry_run {
        logger::info("Dry-run enabled: output and cache were not written");
        return Ok(stats);
    }

    if !tracker.needs_update() {
        logger::info("No source changes detected: output was not rewritten");
        return Ok(stats);
    }

    // Cache chỉ được xác nhận sau khi output đã ghi thành công.
    writer::write(files, config)?;
    tracker.save()?;
    stats.output_written = true;

    Ok(stats)
}
