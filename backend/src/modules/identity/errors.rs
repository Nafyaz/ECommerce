use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum IdentityDomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    // Should I return mail sent even if Identity Already Exists?
    #[error("User Identity already exists")]
    IdentityAlreadyExists,

    #[error("User not found")]
    IdentityNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for IdentityDomainError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        IdentityDomainError::InternalError(format!("Database error: {}", err))
    }
}

impl From<IdentityDomainError> for AppError {
    fn from(error: IdentityDomainError) -> Self {
        match error {
            IdentityDomainError::InvalidEmail(msg) => AppError::Validation(msg),
            IdentityDomainError::WeakPassword(msg) => AppError::Validation(msg),
            IdentityDomainError::InternalError(msg) => AppError::Internal(msg),
            IdentityDomainError::InvalidCredentials => AppError::Unauthorized("Invalid email or password".into()),
            IdentityDomainError::IdentityNotFound => AppError::NotFound("User not found".into()),
            IdentityDomainError::IdentityAlreadyExists => {
                AppError::Conflict("User identity already exists with this email".into())
            }
        }
    }
}
