use crate::modules::identity::domain::OtpDomainError;
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct OtpCode(SecretString);

impl OtpCode {
    const OTP_LENGTH: usize = 6;

    pub fn new(plain: SecretString) -> Result<Self, OtpDomainError> {
        let plain_str = plain.expose_secret();

        if plain_str.len() != Self::OTP_LENGTH || !plain_str.chars().all(|c| c.is_ascii_digit()) {
            return Err(OtpDomainError::InvalidOtpCode);
        }

        Ok(Self(plain.into()))
    }
    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}
