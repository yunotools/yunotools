use yunotools_core::{AppContext, ModuleRegistry};

use yunotools_copyast::CopyastCommand;

fn main() {
    let ctx = AppContext::new("yunotools".into());

    let mut registry = ModuleRegistry::new();

    registry.register(Box::new(CopyastCommand::new()));

    let raw_args = std::env::args().collect::<Vec<_>>();

    if let Err(err) = registry.execute(&ctx, &raw_args) {
        eprintln!("{}", err);
    }
}
