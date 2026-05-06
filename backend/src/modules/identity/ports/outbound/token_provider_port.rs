use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Claim, IdentityId, TokenType};
use thiserror::Error;

pub trait TokenProviderPort: Send + Sync {
    fn generate_token(&self, user_id: &IdentityId, token_type: &TokenType) -> Result<String, TokenProviderError>;
    fn validate_token(&self, token: &str) -> Result<Claim, TokenProviderError>;
}

#[derive(Error, Debug)]
pub enum TokenProviderError {
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

impl From<TokenProviderError> for IdentityError {
    fn from(error: TokenProviderError) -> Self {
        match error {
            TokenProviderError::FailedGeneration => {
                IdentityError::InternalError("Failed to generate token".to_string())
            }
            TokenProviderError::Expired
            | TokenProviderError::InvalidSignature
            | TokenProviderError::Malformed
            | TokenProviderError::TypeMismatch { .. }
            | TokenProviderError::MissingToken
            | TokenProviderError::InvalidToken => IdentityError::InvalidCredentials,
        }
    }
}
