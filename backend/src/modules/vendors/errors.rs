use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VendorDomainError {
    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Vendor not found")]
    VendorNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for VendorDomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => VendorDomainError::VendorNotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                VendorDomainError::InternalError("An internal database error occurred".to_string())
            }
        }
    }
}

impl From<VendorDomainError> for AppError {
    fn from(error: VendorDomainError) -> Self {
        match error {
            VendorDomainError::InvalidName(name) => AppError::Validation(format!("Invalid name: {}", name)),
            VendorDomainError::VendorNotFound => AppError::NotFound("Vendor not found".into()),
            VendorDomainError::InternalError(msg) => AppError::Internal(msg),
        }
    }
}
