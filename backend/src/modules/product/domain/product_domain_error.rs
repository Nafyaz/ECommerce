use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ProductDomainError {
    #[error("invalid product name: {0}")]
    InvalidProductName(String),

    #[error("invalid product price: {0}")]
    InvalidProductPrice(String),

    #[error("invalid timestamps")]
    InvalidTimestamps,
}
