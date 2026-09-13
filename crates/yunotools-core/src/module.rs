use crate::{AppContext, AppError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleMetadata {
    /// Tên duy nhất dùng để hiển thị và tìm kiếm module.
    pub name: &'static str,

    /// Mô tả ngắn về chức năng của module.
    pub description: &'static str,

    /// Các cờ CLI có thể dùng để chọn module.
    pub command_flags: &'static [&'static str],
}

pub trait ToolModule {
    fn metadata(&self) -> ModuleMetadata;

    fn can_handle(&self, raw_args: &[String]) -> bool {
        let metadata = self.metadata();

        raw_args
            .iter()
            .any(|arg| metadata.command_flags.contains(&arg.as_str()))
    }

    fn execute(&self, context: &AppContext, raw_args: &[String]) -> Result<(), AppError>;
}
