use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductRepositoryError {
    #[error("Product persistence unavailable")]
    Unavailable,

    #[error("Product not found")]
    NotFound,

    #[error("Product persistence conflict")]
    Conflict,

    #[error("Corrupt product data: {0}")]
    CorruptData(String),

    #[error("Unexpected product persistence error")]
    Unexpected,
}
