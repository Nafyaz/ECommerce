use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Claim, IdentityId};
use thiserror::Error;

pub trait TokenServicePort: Send + Sync {
    fn generate_token(&self, user_id: &IdentityId) -> Result<String, TokenServiceError>;
    fn validate_token(&self, token: &str) -> Result<Claim, TokenServiceError>;
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
