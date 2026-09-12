use crate::CopyastService;

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

            description: "Copy files into one context",
        }
    }

    fn execute(&self, _ctx: &AppContext, args: &[String]) -> Result<(), AppError> {
        let input = args.get(1).map(|x| x.as_str()).unwrap_or(".");

        let output = args
            .get(2)
            .map(|x| x.as_str())
            .unwrap_or("copyast-output.txt");

        self.service
            .run(input, output)
            .map_err(|e| AppError::Module(e.to_string()))
    }
}
