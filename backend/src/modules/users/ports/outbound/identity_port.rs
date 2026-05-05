use crate::modules::users::domain::value_objects::AccountId;
use async_trait::async_trait;
use thiserror::Error;

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
    async fn check_verified(&self, account_id: &AccountId) -> Result<bool, IdentityPortError>;
}
