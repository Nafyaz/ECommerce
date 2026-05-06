use crate::modules::vendors::domain::value_objects::{OwnerId, VendorId};
use crate::modules::vendors::errors::VendorDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait VendorQueryPort: Send + Sync + 'static {
    async fn check_ownership(&self, id: &VendorId, owner_id: &OwnerId) -> Result<bool, VendorDomainError>;
}
