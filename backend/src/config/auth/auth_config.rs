use crate::config::auth::{AuthConfigDto, AuthConfigError};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

#[derive(Debug)]
pub struct AuthConfig {
    jwt_secret: SecretString,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
    otp_secret: SecretString,
}

impl AuthConfig {
    pub fn jwt_secret(&self) -> &SecretString {
        &self.jwt_secret
    }

    pub fn access_token_ttl(&self) -> Duration {
        self.access_token_ttl
    }

    pub fn refresh_token_ttl(&self) -> Duration {
        self.refresh_token_ttl
    }

    pub fn otp_secret(&self) -> &SecretString {
        &self.otp_secret
    }
}

impl TryFrom<AuthConfigDto> for AuthConfig {
    type Error = AuthConfigError;

    fn try_from(auth_config_dto: AuthConfigDto) -> Result<Self, Self::Error> {
        if auth_config_dto.access_token_ttl.is_zero() {
            return Err(AuthConfigError::ZeroAccessTtl(auth_config_dto.access_token_ttl));
        }

        if auth_config_dto.refresh_token_ttl < auth_config_dto.access_token_ttl {
            return Err(AuthConfigError::InvalidTtlOrder {
                access_token_ttl: auth_config_dto.access_token_ttl,
                refresh_token_ttl: auth_config_dto.refresh_token_ttl,
            });
        }

        let jwt_secret_len = auth_config_dto.jwt_secret.expose_secret().len();
        if auth_config_dto.jwt_secret.expose_secret().len() < 32 {
            return Err(AuthConfigError::WeakJwtSecret(jwt_secret_len));
        }

        let otp_secret_len = auth_config_dto.otp_secret.expose_secret().len();
        if auth_config_dto.otp_secret.expose_secret().len() < 32 {
            return Err(AuthConfigError::WeakOtpSecret(otp_secret_len));
        }

        Ok(Self {
            jwt_secret: auth_config_dto.jwt_secret,
            access_token_ttl: auth_config_dto.access_token_ttl,
            refresh_token_ttl: auth_config_dto.refresh_token_ttl,
            otp_secret: auth_config_dto.otp_secret,
        })
    }
}
