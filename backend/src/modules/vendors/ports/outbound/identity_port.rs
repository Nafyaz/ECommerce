use crate::modules::vendors::OwnerId;
use crate::modules::vendors::errors::VendorDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityPort: Send + Sync {
    // TODO: IdentityPort should not know about VendorDomainError
    async fn is_verified(&self, id: OwnerId) -> Result<bool, VendorDomainError>;
}
