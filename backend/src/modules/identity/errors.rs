use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum IdentityError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    // Should I return to user that mail is sent give otp even if Identity Already Exists?
    #[error("User Identity already exists")]
    IdentityAlreadyExists,

    #[error("User not found")]
    IdentityNotFound,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for IdentityError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        IdentityError::InternalError(format!("Database error: {}", err))
    }
}

impl From<IdentityError> for AppError {
    fn from(error: IdentityError) -> Self {
        match error {
            IdentityError::InvalidEmail(msg) => AppError::Validation(msg),
            IdentityError::WeakPassword(msg) => AppError::Validation(msg),
            IdentityError::InternalError(msg) => AppError::Internal(msg),
            IdentityError::InvalidCredentials => AppError::Unauthorized("Invalid email or password".into()),
            IdentityError::IdentityNotFound => AppError::NotFound("User not found".into()),
            IdentityError::IdentityAlreadyExists => {
                AppError::Conflict("User identity already exists with this email".into())
            }
        }
    }
}
