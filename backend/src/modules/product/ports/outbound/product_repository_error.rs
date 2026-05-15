use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductRepositoryError {
    #[error("product not found")]
    NotFound,

    #[error("persistence operation failed")]
    OperationFailed,
}
