use crate::{
    CopyastConfig, CopyastResult, CopyastService, DEFAULT_MAX_FILE_SIZE, IgnoreGenerator,
    IgnoreTemplate, PathMode, TokenModel,
};
use clap::error::ErrorKind;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use yunotools_core::{AppContext, AppError, LogLevel, ModuleMetadata, ToolModule, logger};

#[derive(Default)]
pub struct CopyastCommand {
    service: CopyastService,
}

impl CopyastCommand {
    pub fn new() -> Self {
        Self::default()
    }

    fn run(&self, options: CopyastArguments) -> Result<(), AppError> {
        configure_logging(&options);

        if options.list_templates {
            print_templates();
            return Ok(());
        }

        if let Some(template_name) = &options.generate_ignore {
            if options.input.is_some() || options.output.is_some() {
                return Err(module_error("--gen-ignore does not accept INPUT or OUTPUT"));
            }

            IgnoreGenerator::generate_to(template_name, &options.ignore_output)
                .map_err(|error| module_error(error.to_string()))?;

            logger::success(&format!(
                "Generated {} from template `{template_name}`",
                options.ignore_output.display(),
            ));
            return Ok(());
        }

        if !options.copyast {
            return Err(module_error("Copyast requires -c or --copyast"));
        }

        let input = options.input.unwrap_or_else(|| PathBuf::from("."));
        let output = normalize_output_path(
            options
                .output
                .unwrap_or_else(|| PathBuf::from("copyast-output.txt")),
        );

        let config = CopyastConfig::new(input, output)
            .with_path_mode(options.path_mode.into())
            .with_token_model(options.token_model.into())
            .with_incremental(options.incremental)
            .with_deduplication(options.deduplicate)
            .with_dry_run(options.dry_run)
            .with_ignore_files(!options.no_ignore)
            .with_hidden_files(options.hidden)
            .with_additional_ignore_files(options.additional_ignore_files)
            .with_max_file_size(options.max_file_size);

        let result = self
            .service
            .run(&config)
            .map_err(|error| module_error(error.to_string()))?;

        print_summary(&result, config.dry_run);
        Ok(())
    }
}

impl ToolModule for CopyastCommand {
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            name: "copyast",
            description: "Copy text files into one AI context file",
        }
    }

    fn execute(&self, _context: &AppContext, args: &[String]) -> Result<(), AppError> {
        let options = match CopyastArguments::try_parse_from(args) {
            Ok(options) => options,
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
                ) =>
            {
                error
                    .print()
                    .map_err(|print_error| module_error(print_error.to_string()))?;
                return Ok(());
            }
            Err(error) => return Err(module_error(error.to_string())),
        };

        self.run(options)
    }

    fn matches(&self, args: &[String]) -> bool {
        args.iter().any(|argument| {
            matches!(
                argument.as_str(),
                "-c" | "--copyast" | "--gen-ignore" | "--list-templates"
            )
        })
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "yuntuns",
    about = "Copy text source files into one context file for AI",
    disable_version_flag = true
)]
struct CopyastArguments {
    /// Run the Copyast module.
    #[arg(short = 'c', long = "copyast", conflicts_with_all = ["generate_ignore", "list_templates"])]
    copyast: bool,

    /// Generate ignore rules. Combine templates with comma: rust,tauri.
    #[arg(
        long = "gen-ignore",
        value_name = "TEMPLATE",
        conflicts_with = "list_templates"
    )]
    generate_ignore: Option<String>,

    /// Print every available language and framework template.
    #[arg(long, conflicts_with = "generate_ignore")]
    list_templates: bool,

    /// File or directory to scan. Defaults to the current directory.
    #[arg(value_name = "INPUT")]
    input: Option<PathBuf>,

    /// Output file or directory. A directory receives copyast-output.txt.
    #[arg(value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Destination used by --gen-ignore.
    #[arg(long, default_value = ".yunotools-ignore", value_name = "FILE")]
    ignore_output: PathBuf,

    /// Path format written in each file header.
    #[arg(long, value_enum, default_value_t = PathModeArgument::Auto)]
    path_mode: PathModeArgument,

    /// AI tokenizer family used for token estimation.
    #[arg(long, value_enum, default_value_t = TokenModelArgument::O200k)]
    token_model: TokenModelArgument,

    /// Only rewrite output when sources or rendering configuration changed.
    #[arg(long)]
    incremental: bool,

    /// Keep only the first file in each exact duplicate group.
    #[arg(long)]
    deduplicate: bool,

    /// Scan and report without writing output or incremental cache.
    #[arg(long)]
    dry_run: bool,

    /// Disable .gitignore, .dockerignore, .helmignore and similar files.
    #[arg(long)]
    no_ignore: bool,

    /// Include hidden files and directories.
    #[arg(long)]
    hidden: bool,

    /// Add another gitignore-compatible filename. May be repeated.
    #[arg(long = "ignore-file", value_name = "NAME")]
    additional_ignore_files: Vec<String>,

    /// Maximum accepted size of one source file, in bytes.
    #[arg(long, default_value_t = DEFAULT_MAX_FILE_SIZE, value_name = "BYTES")]
    max_file_size: u64,

    /// Enable debug logs. YUNTUNS_LOG=debug provides the same behavior.
    #[arg(short, long, conflicts_with = "quiet")]
    verbose: bool,

    /// Disable application logs.
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PathModeArgument {
    Auto,
    Relative,
    Absolute,
}

impl From<PathModeArgument> for PathMode {
    fn from(value: PathModeArgument) -> Self {
        match value {
            PathModeArgument::Auto => Self::Auto,
            PathModeArgument::Relative => Self::Relative,
            PathModeArgument::Absolute => Self::Absolute,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum TokenModelArgument {
    Cl100k,
    O200k,
    Claude,
    Gemini,
}

impl From<TokenModelArgument> for TokenModel {
    fn from(value: TokenModelArgument) -> Self {
        match value {
            TokenModelArgument::Cl100k => Self::Cl100k,
            TokenModelArgument::O200k => Self::O200k,
            TokenModelArgument::Claude => Self::Claude,
            TokenModelArgument::Gemini => Self::Gemini,
        }
    }
}

fn configure_logging(options: &CopyastArguments) {
    if options.verbose {
        logger::set_level(LogLevel::Debug);
    } else if options.quiet {
        logger::set_level(LogLevel::Off);
    }
}

fn normalize_output_path(output: PathBuf) -> PathBuf {
    if output.is_dir() {
        output.join("copyast-output.txt")
    } else {
        output
    }
}

fn print_templates() {
    println!("Available ignore templates:");

    for template_name in IgnoreTemplate::available() {
        println!("  {template_name}");
    }
}

fn print_summary(result: &CopyastResult, dry_run: bool) {
    if dry_run {
        logger::success("Dry run completed; no output was written");
    } else {
        logger::success(&format!("Output: {}", result.output.display()));
    }

    logger::info(&format!(
        "Files: {} copied, {} skipped; context: {} bytes, approximately {} tokens",
        result.scan.copied,
        result.scan.skipped(),
        result.total_bytes,
        result.estimated_tokens,
    ));

    if !result.detected_languages.is_empty() {
        logger::info(&format!(
            "Languages: {}",
            result.detected_languages.join(", ")
        ));
    }

    if !result.detected_frameworks.is_empty() {
        logger::info(&format!(
            "Frameworks: {}",
            result.detected_frameworks.join(", ")
        ));
    }

    if !result.duplicate_groups.is_empty() {
        logger::warn(&format!(
            "Found {} exact duplicate group(s); use --deduplicate to remove copies",
            result.duplicate_groups.len(),
        ));
    }

    if let Some(incremental) = &result.incremental {
        logger::info(&format!(
            "Incremental: {} changed, {} unchanged, {} removed, output written: {}",
            incremental.changed,
            incremental.unchanged,
            incremental.removed,
            incremental.output_written,
        ));
    }
}

fn module_error(message: impl Into<String>) -> AppError {
    AppError::Module(message.into())
}
