use crate::modules::shared::AppError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ImageError {
    #[error("invalid content type: {0}")]
    InvalidContentType(String),

    #[error("file size {size} must be between {min} and {max} bytes")]
    InvalidSize { size: i64, min: i64, max: i64 },

    #[error("invalid object key: {0}")]
    InvalidObjectKey(String),

    #[error("image not found")]
    NotFound,

    #[error("image is not in pending status")]
    NotPending,

    #[error("storage operation failed: {0}")]
    StorageFailure(String),

    #[error("persistence operation failed: {0}")]
    PersistenceFailure(String),

    #[error("invalid product image status: {0}")]
    InvalidProductImageStatus(String),

    #[error("invalid timestamps: created_at must be before updated_at")]
    InvalidTimestamps,

    #[error("invalid state")]
    InvalidState,
}

impl From<ImageError> for AppError {
    fn from(error: ImageError) -> Self {
        AppError::Internal(error.to_string())
    }
}

#[derive(Error, Debug, Clone)]
pub enum ProductError {
    #[error("Actor not verified: {0}")]
    ActorNotVerified(Uuid),

    #[error("Identity port error")]
    IdentityPortError,

    #[error("Vendor port error")]
    VendorPortError,

    #[error("Invalid price: {0}")]
    InvalidPrice(String),

    #[error("Vendor not found")]
    VendorNotFound,

    #[error("You do not have access to this vendor")]
    VendorOwnershipMismatch,

    #[error("Internal error: {0}")]
    InternalError(String),
}
impl From<ProductError> for AppError {
    fn from(error: ProductError) -> Self {
        match error {
            ProductError::ActorNotVerified(actor_id) => {
                AppError::Forbidden(format!("Actor not verified: {}", actor_id))
            }
            ProductError::IdentityPortError => AppError::Internal("Identity port error".into()),
            ProductError::VendorPortError => AppError::Internal("Vendor port error".into()),
            ProductError::InvalidPrice(message) => AppError::Validation(message),
            ProductError::VendorNotFound => AppError::NotFound("Vendor not found".into()),
            ProductError::VendorOwnershipMismatch => {
                AppError::Forbidden("You do not have access to this vendor".into())
            }
            ProductError::InternalError(message) => AppError::Internal(message),
        }
    }
}
