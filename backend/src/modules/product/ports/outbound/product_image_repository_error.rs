use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductImageRepositoryError {
    #[error("Product image not found")]
    NotFound,

    #[error("Product not found")]
    ProductNotFound,

    #[error("Display order is already used for this product")]
    DisplayOrderConflict,

    #[error("Product image object key already exists")]
    ObjectKeyConflict,

    #[error("Product image persistence unavailable")]
    Unavailable,

    #[error("Corrupt product image data: {0}")]
    CorruptData(String),

    #[error("Unexpected product image persistence error")]
    Unexpected,
}
