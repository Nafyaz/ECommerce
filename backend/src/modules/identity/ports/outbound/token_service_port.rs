use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::IdentityId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: DateTime<Utc>,
}

pub trait TokenServicePort: Send + Sync {
    fn generate_token(&self, user_id: &IdentityId) -> Result<String, TokenServiceError>;
    fn validate_token(&self, token: &str) -> Result<Claims, TokenServiceError>;
}

#[derive(Error, Debug)]
pub enum TokenServiceError {
    #[error("Failed to generate token")]
    FailedGeneration,
    #[error("Missing authentication token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
}

impl From<TokenServiceError> for IdentityError {
    fn from(error: TokenServiceError) -> Self {
        match error {
            TokenServiceError::FailedGeneration => IdentityError::InternalError("Failed to generate token".to_string()),
            TokenServiceError::MissingToken => IdentityError::InvalidCredentials,
            TokenServiceError::InvalidToken => IdentityError::InvalidCredentials,
        }
    }
}
