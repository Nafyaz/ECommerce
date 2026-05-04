use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ProductDomainError {
    #[error("Invalid product name: {0}")]
    InvalidProductName(String),

    #[error("Invalid price: {0}")]
    InvalidPrice(String),

    #[error("Vendor not found")]
    VendorNotFound,

    #[error("You do not have access to this vendor")]
    VendorOwnershipMismatch,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for ProductDomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ProductDomainError::VendorNotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                ProductDomainError::InternalError("An internal database error occurred".to_string())
            }
        }
    }
}

impl From<ProductDomainError> for AppError {
    fn from(error: ProductDomainError) -> Self {
        match error {
            ProductDomainError::InvalidProductName(message) => AppError::Validation(message),
            ProductDomainError::InvalidPrice(message) => AppError::Validation(message),
            ProductDomainError::VendorNotFound => AppError::NotFound("Vendor not found".into()),
            ProductDomainError::VendorOwnershipMismatch => {
                AppError::Forbidden("You do not have access to this vendor".into())
            }
            ProductDomainError::InternalError(message) => AppError::Internal(message),
        }
    }
}
