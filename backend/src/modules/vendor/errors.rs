use crate::modules::shared::AppError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum VendorDomainError {
    #[error("Owner not verified: {0}")]
    OwnerNotVerified(Uuid),

    #[error("Identity port error")]
    IdentityPortError,

    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Vendor not found")]
    VendorNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<VendorDomainError> for AppError {
    fn from(error: VendorDomainError) -> Self {
        match error {
            VendorDomainError::OwnerNotVerified(owner_id) => {
                AppError::Unauthorized(format!("Owner not verified: {}", owner_id))
            }
            VendorDomainError::IdentityPortError => AppError::Internal("Identity port error".into()),
            VendorDomainError::InvalidName(name) => AppError::Validation(format!("Invalid name: {}", name)),
            VendorDomainError::VendorNotFound => AppError::NotFound("Vendor not found".into()),
            VendorDomainError::InternalError(msg) => AppError::Internal(msg),
        }
    }
}
