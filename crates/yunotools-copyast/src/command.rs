use crate::{CopyastService, IgnoreGenerator};

use yunotools_core::{AppContext, AppError, ModuleMetadata, ToolModule};

pub struct CopyastCommand {
    service: CopyastService,
}

impl CopyastCommand {
    pub fn new() -> Self {
        Self {
            service: CopyastService::new(),
        }
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
            if let Some(kind) = args.get(index + 1) {
                IgnoreGenerator::generate(kind).map_err(|e| AppError::Module(e))?;

                return Ok(());
            }
        }

        let input = args.get(2).map(|x| x.as_str()).unwrap_or(".");

        let output = args
            .get(3)
            .map(|x| x.as_str())
            .unwrap_or("copyast-output.txt");

        self.service
            .run(input, output)
            .map_err(|err| AppError::Module(err.to_string()))?;

        Ok(())
    }

    fn matches(&self, args: &[String]) -> bool {
        args.iter()
            .any(|x| x == "-c" || x == "--copyast" || x == "--gen-ignore")
    }
}
