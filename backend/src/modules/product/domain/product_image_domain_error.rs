use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductImageDomainError {
    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("File size {size} must be between {min} and {max} bytes")]
    InvalidSize { size: i64, min: i64, max: i64 },

    #[error("Invalid display order: {0}")]
    InvalidDisplayOrder(i32),

    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Invalid object key: {0}")]
    InvalidObjectKey(String),

    #[error("Invalid state transition")]
    InvalidStateTransition,

    #[error("Invalid timestamps: {0}")]
    InvalidTimestamps(String),
}
