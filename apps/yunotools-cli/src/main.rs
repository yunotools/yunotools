mod args;

use args::Args;

use clap::Parser;

use yunotools_core::{AppContext, ModuleRegistry};

use yunotools_copyast::CopyastCommand;

fn main() {
    let args = Args::parse();

    let ctx = AppContext::new("yunotools".into());

    let mut registry = ModuleRegistry::new();

    registry.register(Box::new(CopyastCommand::new()));

    let raw_args = std::env::args().collect::<Vec<_>>();

    registry.execute(&ctx, &raw_args).unwrap();
}
