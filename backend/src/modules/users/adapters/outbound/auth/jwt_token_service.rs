use crate::modules::shared::AppError;
use crate::modules::users::domain::value_objects::UserId;
use crate::modules::users::ports::outbound::{Claims, TokenServicePort};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};

pub struct JwtTokenService {
    secret: SecretString,
    expiration_hours: u64,
}

impl JwtTokenService {
    pub fn new(secret: SecretString, expiration_hours: u64) -> Self {
        Self {
            secret,
            expiration_hours,
        }
    }
}

// TODO: Learn how JsonWebToken works under the hood
impl TokenServicePort for JwtTokenService {
    fn generate_token(&self, user_id: &UserId, role: &str) -> Result<String, AppError> {
        let now = Utc::now().timestamp() as usize;
        let exp = now + (self.expiration_hours as usize * 3600);

        let claims = Claims {
            sub: user_id.to_string(),
            role: role.to_string(),
            exp,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.expose_secret().as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))?;

        Ok(token)
    }

    fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.expose_secret().as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

        Ok(token_data.claims)
    }
}
