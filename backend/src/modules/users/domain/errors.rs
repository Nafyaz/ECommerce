use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum UserDomainError {
    #[error("Invalid name: {0}")]
    InvalidName(String),

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid phone: {0}")]
    InvalidPhone(String),

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,
}

impl From<UserDomainError> for AppError {
    fn from(error: UserDomainError) -> Self {
        match error {
            UserDomainError::InvalidName(msg) => AppError::Validation(msg),
            UserDomainError::InvalidEmail(msg) => AppError::Validation(msg),
            UserDomainError::InvalidPhone(msg) => AppError::Validation(msg),
            UserDomainError::WeakPassword(msg) => AppError::Validation(msg),
            UserDomainError::InvalidCredentials => AppError::Unauthorized("Invalid email or password".into()),
            UserDomainError::UserNotFound => AppError::NotFound("User not found".into()),
            UserDomainError::UserAlreadyExists => AppError::Conflict("User already exists with this email".into()),
        }
    }
}
