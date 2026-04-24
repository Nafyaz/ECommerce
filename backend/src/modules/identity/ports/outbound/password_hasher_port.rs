use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::domain::value_objects::{Password, PasswordHash};
use thiserror::Error;

pub trait PasswordHasherPort: Send + Sync {
    fn hash_from_plain(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError>;
    fn verify(&self, hash: &PasswordHash, plain_candidate: &Password) -> Result<bool, PasswordHasherError>;
}

#[derive(Error, Debug)]
pub enum PasswordHasherError {
    #[error("Failed to hash password: {0}")]
    FailedHashing(String),
    #[error("Failed to verify password: {0}")]
    FailedVerification(String),
}

impl From<PasswordHasherError> for IdentityDomainError {
    fn from(error: PasswordHasherError) -> Self {
        match error {
            PasswordHasherError::FailedHashing(msg) => IdentityDomainError::InternalError(msg),
            PasswordHasherError::FailedVerification(msg) => IdentityDomainError::InvalidCredentials,
        }
    }
}
