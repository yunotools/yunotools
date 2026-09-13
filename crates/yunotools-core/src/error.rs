use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("module error: {0}")]
    Module(String),

    #[error("no matching command; run `{app_name} --help`")]
    NoMatchingCommand { app_name: String },

    #[error("configuration error")]
    Config,

    #[error("unknown error")]
    Unknown,
}
