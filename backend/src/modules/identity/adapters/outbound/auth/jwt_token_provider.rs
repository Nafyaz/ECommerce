use crate::modules::identity::domain::value_objects::{Claim, IdentityId, TokenType};
use crate::modules::identity::ports::outbound::{TokenProviderError, TokenProviderPort};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

pub struct JwtTokenProvider {
    secret: SecretString,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

impl JwtTokenProvider {
    pub fn new(secret: SecretString, access_token_ttl: Duration, refresh_token_ttl: Duration) -> Self {
        Self {
            secret,
            access_token_ttl,
            refresh_token_ttl,
        }
    }

    fn ttl_for(&self, token_type: &TokenType) -> Duration {
        match token_type {
            TokenType::Access => self.access_token_ttl,
            TokenType::Refresh => self.refresh_token_ttl,

            // TODO: use different TTLs for different token types??
            TokenType::PasswordChange | TokenType::EmailChange | TokenType::DeleteAccount => self.access_token_ttl,
        }
    }
}

// TODO: Learn how JsonWebToken works under the hood
impl TokenProviderPort for JwtTokenProvider {
    fn generate_token(&self, identity_id: &IdentityId, token_type: &TokenType) -> Result<String, TokenProviderError> {
        let now = Utc::now();
        let expiration = now + self.ttl_for(token_type);

        let claims = Claim {
            sub: identity_id.as_uuid().to_owned(),
            token_type: token_type.as_str().to_owned(),
            iat: now.timestamp(),
            exp: expiration.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.expose_secret().as_bytes()),
        )
        .map_err(|_| TokenProviderError::FailedGeneration)?;

        Ok(token)
    }

    fn validate_token(&self, token: &str) -> Result<Claim, TokenProviderError> {
        let token_data = decode::<Claim>(
            token,
            &DecodingKey::from_secret(self.secret.expose_secret().as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::error!("JWT validation failed: {:?}", e);
            TokenProviderError::InvalidSignature
        })?;

        tracing::debug!("Token data: {:?}", token_data);

        Ok(token_data.claims)
    }
}
