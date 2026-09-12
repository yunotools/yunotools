use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("module error: {0}")]
    Module(String),

    #[error("configuration error")]
    Config,

    #[error("unknown error")]
    Unknown,
}
