use crate::modules::notifications::NotificationError;
use crate::modules::shared::AppError;
use thiserror::Error;

// TODO: Rename to IdentityDomainError
#[derive(Error, Debug, Clone)]
pub enum IdentityError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Invalid OTP")]
    InvalidOtp,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Email not verified")]
    EmailNotVerified,

    #[error("Invalid identity status {0}")]
    InvalidIdentityStatus(String),

    // Should I return to user that mail is sent give otp even if Identity Already Exists?
    #[error("User Identity already exists")]
    VerifiedIdentityAlreadyExists,

    #[error("User not found")]
    IdentityNotFound,

    #[error("Invalid OTP purpose: {0}")]
    InvalidOtpPurpose(String),

    #[error("Invalid OTP status: {0}")]
    InvalidOtpStatus(String),

    #[error("No Active OTP found")]
    NoActiveOtp,

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for IdentityError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        IdentityError::InternalError(format!("Database error: {}", err))
    }
}

impl From<NotificationError> for IdentityError {
    fn from(err: NotificationError) -> Self {
        IdentityError::InternalError(format!("Notification error: {}", err))
    }
}

impl From<IdentityError> for AppError {
    fn from(error: IdentityError) -> Self {
        match error {
            IdentityError::InvalidEmail(msg) => AppError::Validation(msg),
            IdentityError::WeakPassword(msg) => AppError::Validation(msg),
            IdentityError::InvalidOtp => AppError::Validation("Invalid OTP".to_owned()),
            IdentityError::InternalError(msg) => AppError::Internal(msg),
            IdentityError::InvalidCredentials => AppError::Unauthorized("Invalid email or password".to_owned()),
            IdentityError::EmailNotVerified => AppError::Unauthorized("Email not verified".to_owned()),
            IdentityError::IdentityNotFound => AppError::NotFound("User not found".to_owned()),
            IdentityError::VerifiedIdentityAlreadyExists => {
                AppError::Conflict("A validated user identity already exists with this email".to_owned())
            }
            IdentityError::InvalidIdentityStatus(msg) => {
                AppError::Internal(format!("Invalid identity status: {}", msg))
            }
            IdentityError::InvalidOtpPurpose(msg) => AppError::Internal(format!("Invalid OTP purpose: {}", msg)),
            IdentityError::InvalidOtpStatus(msg) => AppError::Internal(format!("Invalid OTP status: {}", msg)),
            IdentityError::NoActiveOtp => AppError::Internal("No active OTP found".to_owned()),
        }
    }
}
