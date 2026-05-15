use crate::modules::identity::domain::{IdentityDomainError, OtpDomainError};
use crate::modules::identity::ports::outbound::{
    IdentityRepositoryError, NotificationPortError, OtpProviderError, OtpRepositoryError, PasswordHasherError,
    TokenProviderError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityAppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("Verified identity already exists")]
    VerifiedIdentityAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("email is not verified")]
    EmailNotVerified,

    #[error("invalid otp")]
    InvalidOtp,

    #[error("no active otp found")]
    NoActiveOtp,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Identity not found")]
    IdentityNotFound,

    #[error("{0} dependency unavailable")]
    DependencyUnavailable(&'static str),

    #[error("persistence unavailable")]
    PersistenceUnavailable,

    #[error("internal error")]
    Internal,
}

impl From<IdentityDomainError> for IdentityAppError {
    fn from(err: IdentityDomainError) -> Self {
        match err {
            IdentityDomainError::InvalidEmail(_)
            | IdentityDomainError::InvalidIdentityStatus(_)
            | IdentityDomainError::WeakPassword(_) => Self::InvalidInput(err.to_string()),
            IdentityDomainError::InvalidStateTransition | IdentityDomainError::InvalidTimestamps(_) => {
                Self::Conflict(err.to_string())
            }
        }
    }
}

impl From<OtpDomainError> for IdentityAppError {
    fn from(err: OtpDomainError) -> Self {
        match err {
            OtpDomainError::InvalidOtpPurpose(_)
            | OtpDomainError::InvalidOtpCode
            | OtpDomainError::InvalidOtpStatus(_) => Self::InvalidInput(err.to_string()),
            OtpDomainError::InvalidStateTransition | OtpDomainError::InvalidTimestamps(_) => {
                Self::Conflict(err.to_string())
            }
        }
    }
}

impl From<IdentityRepositoryError> for IdentityAppError {
    fn from(err: IdentityRepositoryError) -> Self {
        match err {
            IdentityRepositoryError::NotFound => Self::IdentityNotFound,
            IdentityRepositoryError::Conflict => Self::Conflict(err.to_string()),
            IdentityRepositoryError::Unavailable => Self::PersistenceUnavailable,
            IdentityRepositoryError::CorruptData(_) | IdentityRepositoryError::Unexpected => Self::Internal,
        }
    }
}

impl From<OtpRepositoryError> for IdentityAppError {
    fn from(err: OtpRepositoryError) -> Self {
        match err {
            OtpRepositoryError::NotFound => Self::NoActiveOtp,
            OtpRepositoryError::Conflict => Self::Conflict(err.to_string()),
            OtpRepositoryError::Unavailable => Self::PersistenceUnavailable,
            OtpRepositoryError::CorruptData(_) | OtpRepositoryError::Unexpected => Self::Internal,
        }
    }
}

impl From<PasswordHasherError> for IdentityAppError {
    fn from(_err: PasswordHasherError) -> Self {
        Self::Internal
    }
}

impl From<TokenProviderError> for IdentityAppError {
    fn from(_err: TokenProviderError) -> Self {
        Self::Internal
    }
}

impl From<OtpProviderError> for IdentityAppError {
    fn from(_err: OtpProviderError) -> Self {
        Self::Internal
    }
}

impl From<NotificationPortError> for IdentityAppError {
    fn from(err: NotificationPortError) -> Self {
        match err {
            NotificationPortError::UnsupportedPurpose => Self::InvalidInput(err.to_string()),
            NotificationPortError::DeliveryFailed => Self::DependencyUnavailable("notification"),
            NotificationPortError::InvalidMessage => Self::Internal,
        }
    }
}
