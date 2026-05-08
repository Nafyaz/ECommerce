use crate::modules::product::domain::value_objects::ProductActorId;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProductIdentityPortError {
    #[error("identity not found")]
    NotFound,

    #[error("identity service unavailable")]
    Unavailable,

    #[error("unexpected identity error")]
    Unexpected,
}

#[async_trait]
pub trait ProductIdentityPort: Send + Sync {
    async fn check_verified(&self, actor_id: ProductActorId) -> Result<bool, ProductIdentityPortError>;
}
