use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::domain::value_objects::VendorId;
use crate::modules::vendors::errors::VendorDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait VendorRepositoryPort: Send + Sync {
    async fn save(&self, vendor: &Vendor) -> Result<(), VendorDomainError>;

    async fn find_by_id(&self, id: &VendorId) -> Result<Option<Vendor>, VendorDomainError>;

    async fn find_all(&self) -> Result<Vec<Vendor>, VendorDomainError>;

    async fn delete(&self, id: &VendorId) -> Result<(), VendorDomainError>;
}
