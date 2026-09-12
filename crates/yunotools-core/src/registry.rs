use crate::{AppContext, AppError, ToolModule};

pub struct ModuleRegistry {
    modules: Vec<Box<dyn ToolModule>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn register(&mut self, module: Box<dyn ToolModule>) {
        self.modules.push(module);
    }

    pub fn execute(&self, ctx: &AppContext, args: &[String]) -> Result<(), AppError> {
        for module in &self.modules {
            module.execute(ctx, args)?;
        }
        Ok(())
    }
}
