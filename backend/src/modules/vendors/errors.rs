use crate::modules::shared::AppError;
use thiserror::Error;

// TODO: Should names be unique?
#[derive(Error, Debug)]
pub enum VendorDomainError {
    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Vendor not found")]
    VendorNotFound,
}

impl From<VendorDomainError> for AppError {
    fn from(error: VendorDomainError) -> Self {
        match error {
            VendorDomainError::InvalidName(name) => AppError::Validation(format!("Invalid name: {}", name)),
            VendorDomainError::VendorNotFound => AppError::NotFound("Vendor not found".into()),
        }
    }
}
