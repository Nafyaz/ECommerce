use crate::modules::identity::domain::value_objects::{Claim, IdentityId};
use crate::modules::identity::ports::outbound::{TokenServiceError, TokenServicePort};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

pub struct JwtTokenService {
    secret: SecretString,
    duration: Duration,
}

impl JwtTokenService {
    pub fn new(secret: SecretString, duration: Duration) -> Self {
        Self { secret, duration }
    }
}

// TODO: Learn how JsonWebToken works under the hood
impl TokenServicePort for JwtTokenService {
    fn generate_token(&self, identity_id: &IdentityId, purpose: &) -> Result<String, TokenServiceError> {
        let now = Utc::now();
        let expiration = now + self.duration;

        let claims = Claim {
            sub: identity_id.as_uuid().to_owned(),
            purpose:
            iat: now,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.expose_secret().as_bytes()),
        )
            .map_err(|_| TokenServiceError::FailedGeneration)?;

        Ok(token)
    }

    fn validate_token(&self, token: &str) -> Result<Claim, TokenServiceError> {
        let token_data = decode::<Claim>(
            token,
            &DecodingKey::from_secret(self.secret.expose_secret().as_bytes()),
            &Validation::default(),
        )
            .map_err(|e| TokenServiceError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
