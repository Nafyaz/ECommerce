use super::AuthConfigError;
use super::AuthConfigError::{InvalidTtlOrder, WeakJwtSecret, WeakOtpSecret, ZeroAccessTtl};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthConfigDto {
    pub jwt_secret: SecretString,

    #[serde(with = "humantime_serde")]
    pub access_token_ttl: Duration,

    #[serde(with = "humantime_serde")]
    pub refresh_token_ttl: Duration,

    pub otp_secret: SecretString,
}

impl AuthConfigDto {
    pub fn validate(&self) -> Result<(), AuthConfigError> {
        if self.access_token_ttl.is_zero() {
            return Err(ZeroAccessTtl(self.access_token_ttl));
        }

        if self.refresh_token_ttl < self.access_token_ttl {
            return Err(InvalidTtlOrder {
                access_token_ttl: self.access_token_ttl,
                refresh_token_ttl: self.refresh_token_ttl,
            });
        }

        let jwt_secret_len = self.jwt_secret.expose_secret().len();
        if self.jwt_secret.expose_secret().len() < 32 {
            return Err(WeakJwtSecret(jwt_secret_len));
        }

        let otp_secret_len = self.otp_secret.expose_secret().len();
        if self.otp_secret.expose_secret().len() < 32 {
            return Err(WeakOtpSecret(otp_secret_len));
        }

        Ok(())
    }
}
