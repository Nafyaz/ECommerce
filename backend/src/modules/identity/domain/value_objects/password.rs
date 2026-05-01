use crate::modules::identity::IdentityError;
use secrecy::{ExposeSecret, SecretString};

pub struct Password(SecretString);

impl Password {
    const MIN_PASSWORD_LENGTH: usize = 8;

    pub fn new(plain: SecretString) -> Result<Self, IdentityError> {
        let plain_str = plain.expose_secret();

        Self::validate_strength(plain_str)?;

        Ok(Self(plain))
    }

    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }

    fn validate_strength(plain: &str) -> Result<(), IdentityError> {
        if plain.len() < Self::MIN_PASSWORD_LENGTH {
            return Err(IdentityError::WeakPassword(format!(
                "Password must be at least {} characters",
                Self::MIN_PASSWORD_LENGTH
            )));
        }

        let has_uppercase = plain.chars().any(|c| c.is_uppercase());
        let has_lowercase = plain.chars().any(|c| c.is_lowercase());
        let has_digit = plain.chars().any(|c| c.is_ascii_digit());

        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(IdentityError::WeakPassword(
                "Password must contain uppercase, lowercase, and a digit".to_string(),
            ));
        }

        Ok(())
    }
}
