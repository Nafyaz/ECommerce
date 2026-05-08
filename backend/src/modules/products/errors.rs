use crate::modules::shared::AppError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Clone)]
pub enum ProductDomainError {
    #[error("Actor not verified: {0}")]
    ActorNotVerified(Uuid),

    #[error("Identity port error")]
    IdentityPortError,

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
impl From<ProductDomainError> for AppError {
    fn from(error: ProductDomainError) -> Self {
        match error {
            ProductDomainError::ActorNotVerified(actor_id) => {
                AppError::Forbidden(format!("Actor not verified: {}", actor_id))
            }
            ProductDomainError::IdentityPortError => AppError::Internal("Identity port error".into()),
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
