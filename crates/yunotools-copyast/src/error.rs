use thiserror::Error;

#[derive(Debug, Error)]
pub enum CopyastError {
    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("invalid path")]
    InvalidPath,
}
