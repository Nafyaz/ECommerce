use crate::modules::notification::NotificationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationPortError {
    #[error("invalid notification message")]
    InvalidMessage,

    #[error("otp purpose is not supported by email notification")]
    UnsupportedPurpose,

    #[error("notification delivery failed")]
    DeliveryFailed,
}

impl From<NotificationError> for NotificationPortError {
    fn from(err: NotificationError) -> Self {
        match err {
            NotificationError::DeliveryFailed(_) => Self::DeliveryFailed,
            NotificationError::InvalidSubject(_)
            | NotificationError::InvalidBody(_)
            | NotificationError::InvalidRecipient(_)
            | NotificationError::InvalidChannel(_) => Self::InvalidMessage,
        }
    }
}
