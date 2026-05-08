use crate::modules::product::domain::value_objects::{ProductActorId, SupplierId};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProductVendorPortError {
    #[error("vendor not found")]
    NotFound,

    #[error("vendor service unavailable")]
    Unavailable,

    #[error("unexpected vendor error")]
    Unexpected,
}

#[async_trait]
pub trait ProductVendorPort: Send + Sync {
    async fn check_ownership(
        &self,
        supplier_id: SupplierId,
        actor_id: ProductActorId,
    ) -> Result<bool, ProductVendorPortError>;
}
