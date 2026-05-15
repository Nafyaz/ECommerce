use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductImageDomainError {
    #[error("invalid content type: {0}")]
    InvalidContentType(String),

    #[error("file size {size} must be between {min} and {max} bytes")]
    InvalidSize { size: i64, min: i64, max: i64 },

    #[error("invalid display order: {0}")]
    InvalidDisplayOrder(i32),

    #[error("invalid status: {0}")]
    InvalidStatus(String),

    #[error("invalid state transition")]
    InvalidStateTransition,

    #[error("invalid timestamps")]
    InvalidTimestamps,
}
