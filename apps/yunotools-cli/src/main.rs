use std::process::ExitCode;
use yunotools_core::{AppContext, ModuleRegistry, logger};

use yunotools_copyast::CopyastCommand;

fn main() -> ExitCode {
    logger::init_from_env();

    let ctx = AppContext::new("yuntuns".into());

    let mut registry = ModuleRegistry::new();

    registry.register(Box::new(CopyastCommand::new()));

    let raw_args = std::env::args().collect::<Vec<_>>();

    if wants_global_help(&raw_args) {
        print_global_help();
        return ExitCode::SUCCESS;
    }

    if raw_args.iter().any(|argument| argument == "--version") {
        println!("yuntuns {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    match registry.execute(&ctx, &raw_args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            logger::error(&error.to_string());
            ExitCode::FAILURE
        }
    }
}

fn wants_global_help(args: &[String]) -> bool {
    args.len() == 1
        || (args
            .iter()
            .any(|argument| argument == "-h" || argument == "--help")
            && !args
                .iter()
                .any(|argument| argument == "-c" || argument == "--copyast"))
}

fn print_global_help() {
    println!(
        "yuntuns - colorful terminal tools\n\n\
         Usage:\n  \
           yuntuns -c [INPUT] [OUTPUT] [OPTIONS]\n  \
           yuntuns --gen-ignore <TEMPLATE>\n  \
           yuntuns --list-templates\n\n\
         Run `yuntuns -c --help` for all Copyast options."
    );
}
