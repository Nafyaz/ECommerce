use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum IdentityDomainError {
    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid phone: {0}")]
    InvalidPhone(String),

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,
}

impl From<sqlx::Error> for IdentityDomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => IdentityDomainError::UserNotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                IdentityDomainError::InternalError("An internal database error occurred".to_string())
            }
        }
    }
}

impl From<IdentityDomainError> for AppError {
    fn from(error: IdentityDomainError) -> Self {
        match error {
            IdentityDomainError::InvalidName(msg) => AppError::Validation(msg),
            IdentityDomainError::InvalidEmail(msg) => AppError::Validation(msg),
            IdentityDomainError::InvalidPhone(msg) => AppError::Validation(msg),
            IdentityDomainError::WeakPassword(msg) => AppError::Validation(msg),
            IdentityDomainError::InternalError(msg) => AppError::Internal(msg),
            IdentityDomainError::InvalidCredentials => AppError::Unauthorized("Invalid email or password".into()),
            IdentityDomainError::UserNotFound => AppError::NotFound("User not found".into()),
            IdentityDomainError::UserAlreadyExists => AppError::Conflict("User already exists with this email".into()),
        }
    }
}
