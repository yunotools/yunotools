use crate::{AppContext, AppError};

#[derive(Debug)]
pub struct ModuleMetadata {
    pub name: &'static str,

    pub description: &'static str,
}

pub trait ToolModule {
    fn metadata(&self) -> ModuleMetadata;

    fn execute(&self, ctx: &AppContext, args: &[String]) -> Result<(), AppError>;
}
