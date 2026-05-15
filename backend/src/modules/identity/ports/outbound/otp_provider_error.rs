use crate::modules::identity::domain::OtpDomainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OtpProviderError {
    #[error("failed to generate otp")]
    GenerationFailed,

    #[error("otp hash is malformed")]
    InvalidHash,

    #[error("otp provider is misconfigured")]
    InvalidConfiguration,
}

impl From<OtpDomainError> for OtpProviderError {
    fn from(_err: OtpDomainError) -> Self {
        Self::GenerationFailed
    }
}
