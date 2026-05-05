use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Claim, IdentityId, TokenType};
use thiserror::Error;

pub trait TokenServicePort: Send + Sync {
    fn generate_token(&self, user_id: &IdentityId, token_type: &TokenType) -> Result<String, TokenServiceError>;
    fn validate_token(&self, token: &str) -> Result<Claim, TokenServiceError>;
}

#[derive(Error, Debug)]
pub enum TokenServiceError {
    #[error("Failed to generate token")]
    FailedGeneration,

    #[error("Token has expired")]
    Expired,

    #[error("Token signature is invalid")]
    InvalidSignature,

    #[error("Token format is malformed")]
    Malformed,

    #[error("Token type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Missing authentication token")]
    MissingToken,

    #[error("Invalid token")]
    InvalidToken,
}

impl From<TokenServiceError> for IdentityError {
    fn from(error: TokenServiceError) -> Self {
        match error {
            TokenServiceError::FailedGeneration => IdentityError::InternalError("Failed to generate token".to_string()),
            TokenServiceError::Expired
            | TokenServiceError::InvalidSignature
            | TokenServiceError::Malformed
            | TokenServiceError::TypeMismatch { .. }
            | TokenServiceError::MissingToken
            | TokenServiceError::InvalidToken => IdentityError::InvalidCredentials,
        }
    }
}
