use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ProductDomainError {
    #[error("Invalid product name: {0}")]
    InvalidProductName(String),

    #[error("invalid timestamps")]
    InvalidTimestamps,
}
