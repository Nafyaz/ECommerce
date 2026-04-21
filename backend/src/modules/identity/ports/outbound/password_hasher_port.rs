use crate::modules::identity::domain::value_objects::{Password, PasswordHash};
use crate::modules::shared::AppError;
use secrecy::SecretString;

pub trait PasswordHasherPort: Send + Sync {
    fn hash_from_plain(&self, password: &Password) -> Result<PasswordHash, AppError>;
    fn verify(&self, hash: &PasswordHash, plain_candidate: &Password) -> Result<bool, AppError>;
}
