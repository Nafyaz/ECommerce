use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductRepositoryError {
    #[error("product not found")]
    NotFound,

    #[error("product persistence conflict")]
    Conflict,

    #[error("product persistence unavailable")]
    Unavailable,

    #[error("corrupt product data: {0}")]
    CorruptData(String),

    #[error("unexpected product persistence error")]
    Unexpected,
}
