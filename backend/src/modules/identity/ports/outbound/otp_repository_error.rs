use thiserror::Error;

#[derive(Error, Debug)]
pub enum OtpRepositoryError {
    #[error("Otp persistence unavailable")]
    Unavailable,

    #[error("Otp not found")]
    NotFound,

    #[error("Otp persistence conflict")]
    Conflict,

    #[error("Corrupt otp data: {0}")]
    CorruptData(String),

    #[error("Unexpected persistence error")]
    Unexpected,
}
