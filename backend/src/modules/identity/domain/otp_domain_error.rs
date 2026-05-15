use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum OtpDomainError {
    #[error("Invalid otp purpose: {0}")]
    InvalidOtpPurpose(String),

    #[error("Invalid otp code")]
    InvalidOtpCode,

    #[error("Invalid otp status")]
    InvalidOtpStatus(String),

    #[error("Invalid timestamps: {0}")]
    InvalidTimestamps(String),

    #[error("Invalid state transition")]
    InvalidStateTransition,
}
