use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductImageRepositoryError {
    #[error("product image not found")]
    NotFound,

    #[error("product not found")]
    ProductNotFound,

    #[error("display order is already used for this product")]
    DisplayOrderConflict,

    #[error("product image object key already exists")]
    ObjectKeyConflict,

    #[error("product image persistence unavailable")]
    Unavailable,

    #[error("corrupt product image data: {0}")]
    CorruptData(String),

    #[error("unexpected product image persistence error")]
    Unexpected,
}
