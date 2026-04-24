use crate::modules::identity::domain::value_objects::{Password, PasswordHash};
use crate::modules::identity::ports::outbound::{PasswordHasherError, PasswordHasherPort};
use crate::modules::shared::AppError;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

pub struct Argon2PasswordHasher;

impl PasswordHasherPort for Argon2PasswordHasher {
    fn hash_from_plain(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.expose().as_bytes(), &salt)
            .map_err(|e| PasswordHasherError::FailedHashing(format!("{}", e)))?;

        Ok(PasswordHash::from_str(hash.to_string()))
    }

    fn verify(&self, hash: &PasswordHash, plain_candidate: &Password) -> Result<bool, PasswordHasherError> {
        let parsed_hash = argon2::PasswordHash::new(hash.as_str())
            .map_err(|e| PasswordHasherError::FailedVerification(format!("Password verification failed: {}", e)))?;

        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(plain_candidate.expose().as_bytes(), &parsed_hash)
            .is_ok())
    }
}
