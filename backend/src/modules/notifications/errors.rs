use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum NotificationError {
    #[error("Invalid Subject: {0}")]
    InvalidSubject(String),

    #[error("Invalid Body: {0}")]
    InvalidBody(String),

    #[error("Invalid Recipient: {0}")]
    InvalidRecipient(String),

    #[error("Invalid Channel: {0}")]
    InvalidChannel(String),

    #[error("Delivery failed: {0}")]
    DeliveryFailed(String),
}

impl From<NotificationError> for AppError {
    fn from(error: NotificationError) -> Self {
        match error {
            NotificationError::InvalidSubject(msg) => AppError::Internal(msg),
            NotificationError::InvalidBody(msg) => AppError::Internal(msg),
            NotificationError::InvalidRecipient(msg) => AppError::Internal(msg),
            NotificationError::InvalidChannel(msg) => AppError::Internal(msg),
            NotificationError::DeliveryFailed(msg) => AppError::Internal(msg),
        }
    }
}
