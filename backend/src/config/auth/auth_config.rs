use super::AuthConfigDto;
use super::AuthConfigError;
use secrecy::SecretString;
use std::time::Duration;

#[derive(Debug)]
pub struct AuthConfig {
    jwt_secret: SecretString,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
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
}

impl TryFrom<AuthConfigDto> for AuthConfig {
    type Error = AuthConfigError;

    fn try_from(auth_config_dto: AuthConfigDto) -> Result<Self, Self::Error> {
        auth_config_dto.validate()?;

        Ok(Self {
            jwt_secret: auth_config_dto.jwt_secret,
            access_token_ttl: auth_config_dto.access_token_ttl,
            refresh_token_ttl: auth_config_dto.refresh_token_ttl,
        })
    }
}
