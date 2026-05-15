use crate::modules::identity::domain::IdentityDomainError;
use crate::modules::identity::ports::outbound::IdentityRepositoryError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityAppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("Verified identity already exists")]
    VerifiedIdentityAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Identity not found")]
    IdentityNotFound,
}

impl From<IdentityDomainError> for IdentityAppError {
    fn from(err: IdentityDomainError) -> Self {
        match err {
            IdentityDomainError::InvalidTimestamps(_) => Self::Conflict(err.to_string()),

            _ => Self::InvalidInput(err.to_string()),
        }
    }
}

impl From<IdentityRepositoryError> for IdentityAppError {
    fn from(err: IdentityRepositoryError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}
