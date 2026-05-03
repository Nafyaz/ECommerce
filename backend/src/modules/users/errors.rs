use crate::modules::shared::AppError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Clone)]
pub enum UserDomainError {
    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Invalid phone: {0}")]
    InvalidPhone(String),

    #[error("User already exists {0}")]
    UserAlreadyExists(Uuid),

    #[error("User not found")]
    UserNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for UserDomainError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        UserDomainError::InternalError(format!("Database error: {}", err))
    }
}

impl From<UserDomainError> for AppError {
    fn from(error: UserDomainError) -> Self {
        match error {
            UserDomainError::InvalidName(msg) => AppError::Validation(msg),
            UserDomainError::InvalidPhone(msg) => AppError::Validation(msg),
            UserDomainError::UserAlreadyExists(id) => AppError::Conflict(format!("User already exists: {}", id)),
            UserDomainError::UserNotFound => AppError::NotFound("User not found".into()),
            UserDomainError::InternalError(msg) => AppError::Internal(msg),
        }
    }
}
