use crate::config::auth::auth_config_error::AuthConfigError;
use crate::config::auth::auth_config_error::AuthConfigError::{
    InvalidTtlOrder, WeakJwtSecret, ZeroAccessTtl,
};
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
        let secret_len = self.jwt_secret.expose_secret().len();
        if secret_len < 32 {
            return Err(WeakJwtSecret(secret_len));
        }

        Ok(())
    }
}
