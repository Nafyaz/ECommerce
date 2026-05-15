use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductAppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal error")]
    Internal,
}
