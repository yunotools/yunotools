mod cli;
mod option_catalog;

use std::process::ExitCode;
use yunotools_core::{AppContext, AppError, ModuleMetadata, ModuleRegistry, logger};

use cli::{CliAction, resolve_cli_action, resolve_log_level};
use option_catalog::GLOBAL_OPTIONS;
use yunotools_copyast::CopyastCommand;

const APP_NAME: &str = "yuntuns";

fn main() -> ExitCode {
    logger::init_from_env();

    let raw_args = std::env::args().collect::<Vec<_>>();

    if let Some(log_level) = resolve_log_level(&raw_args) {
        logger::set_level(log_level);
    }

    match run_cli(&raw_args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            logger::error(&error.to_string());
            ExitCode::FAILURE
        }
    }
}

fn run_cli(raw_args: &[String]) -> Result<(), AppError> {
    let registry = create_module_registry();
    let is_module_selected = registry.has_matching_module(raw_args);

    match resolve_cli_action(raw_args, is_module_selected) {
        CliAction::ShowHelp => print_global_help(registry.list_modules()),
        CliAction::ShowVersion => println!("{APP_NAME} {}", env!("CARGO_PKG_VERSION")),
        CliAction::SearchModules { query } => print_search_results(&registry, query.as_deref()),
        CliAction::DispatchModule => {
            let context = AppContext::new(APP_NAME);

            registry.execute(&context, raw_args)?;
        }
    }

    Ok(())
}

fn create_module_registry() -> ModuleRegistry {
    let mut registry = ModuleRegistry::new();
    registry.register_module(CopyastCommand::new());
    registry
}

fn print_search_results(registry: &ModuleRegistry, query: Option<&str>) {
    let modules = match query {
        Some(query) => registry.search_modules(query),
        None => registry.list_modules().to_vec(),
    };

    if modules.is_empty() {
        logger::warn(&format!(
            "No module matched `{}`",
            query.unwrap_or_default()
        ));
        return;
    }

    println!("Available modules:");
    print_modules(&modules);
}

fn print_global_help(modules: &[ModuleMetadata]) {
    println!(
        "{APP_NAME} - colorful terminal tools\n\n\
         Usage:\n  \
           {APP_NAME} -c [INPUT] [OUTPUT] [OPTIONS]\n  \
           {APP_NAME} -c --gen-ignore <TEMPLATE> [-o <FILE>]\n  \
           {APP_NAME} -c --list-templates\n\n\
         Global options:"
    );

    print_global_options();
    println!("\nModules:");
    print_modules(modules);
    println!("\nRun `{APP_NAME} -c --help` for all Copyast options.");
}

fn print_global_options() {
    for option in GLOBAL_OPTIONS {
        let mut usage = option.flags.join(", ");

        if let Some(value_name) = option.value_name {
            usage.push(' ');
            usage.push_str(value_name);
        }

        println!("  {usage:<25} {}", option.description);
    }
}

fn print_modules(modules: &[ModuleMetadata]) {
    for metadata in modules {
        println!(
            "  {:<10} {:<18} {}",
            metadata.name,
            metadata.command_flags.join(", "),
            metadata.description,
        );
    }
}
