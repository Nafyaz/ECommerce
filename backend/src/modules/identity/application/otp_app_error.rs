use crate::modules::identity::domain::OtpDomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OtpAppError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl From<OtpDomainError> for OtpAppError {
    fn from(err: OtpDomainError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}
