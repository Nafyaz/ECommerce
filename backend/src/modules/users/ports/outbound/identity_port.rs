use crate::modules::users::domain::value_objects::AuthIdentityId;
use async_trait::async_trait;
use thiserror::Error;

// TODO: All outbound ports should have their own errors
#[derive(Debug, Error)]
pub enum IdentityPortError {
    #[error("identity not found")]
    NotFound,

    #[error("identity service unavailable")]
    Unavailable,

    #[error("unexpected identity error")]
    Unexpected,
}

#[async_trait]
pub trait IdentityPort: Send + Sync {
    async fn is_verified(&self, auth_identity_id: &AuthIdentityId) -> Result<bool, IdentityPortError>;
}
