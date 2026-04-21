use crate::modules::identity::errors::UserDomainError;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
