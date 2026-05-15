use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Password, PasswordHash};
use thiserror::Error;

pub trait PasswordHasherPort: Send + Sync {
    fn hash_password(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError>;
    fn verify_password(&self, hash: &PasswordHash, plain_candidate: &Password) -> Result<bool, PasswordHasherError>;
}

#[derive(Error, Debug)]
pub enum PasswordHasherError {
    #[error("Failed to hash password: {0}")]
    FailedHashing(String),
    #[error("Failed to verify password: {0}")]
    FailedVerification(String),
}
