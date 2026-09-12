use crate::{CopyastConfig, CopyastService, IgnoreGenerator};

use yunotools_core::{AppContext, AppError, ModuleMetadata, ToolModule};

#[derive(Default)]
pub struct CopyastCommand {
    service: CopyastService,
}

impl CopyastCommand {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToolModule for CopyastCommand {
    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata {
            name: "copyast",
            description: "Copy text files into one context file",
        }
    }

    fn execute(&self, _ctx: &AppContext, args: &[String]) -> Result<(), AppError> {
        if let Some(index) = args.iter().position(|x| x == "--gen-ignore") {
            let kind = args.get(index + 1).ok_or_else(|| {
                AppError::Module("Missing ignore template name after --gen-ignore".into())
            })?;

            IgnoreGenerator::generate(kind).map_err(|error| AppError::Module(error.to_string()))?;
            return Ok(());
        }

        let input = args.get(2).map(|x| x.as_str()).unwrap_or(".");

        let output = args
            .get(3)
            .map(|x| x.as_str())
            .unwrap_or("copyast-output.txt");

        let config = CopyastConfig::new(input, output);

        self.service
            .run(&config)
            .map_err(|error| AppError::Module(error.to_string()))?;

        Ok(())
    }

    fn matches(&self, args: &[String]) -> bool {
        args.iter()
            .any(|x| x == "-c" || x == "--copyast" || x == "--gen-ignore")
    }
}
