use crate::modules::vendor::domain::value_objects::{OwnerId, VendorId};
use crate::modules::vendor::errors::VendorDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait VendorQueryPort: Send + Sync + 'static {
    async fn check_ownership(&self, vendor_id: VendorId, owner_id: OwnerId) -> Result<bool, VendorDomainError>;
}
