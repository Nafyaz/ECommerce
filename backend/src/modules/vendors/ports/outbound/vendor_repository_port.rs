use crate::modules::shared::AppError;
use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::domain::value_objects::VendorId;
use async_trait::async_trait;

#[async_trait]
pub trait VendorRepositoryPort: Send + Sync {
    async fn save(&self, vendor: &Vendor) -> Result<(), AppError>;

    async fn find_by_id(&self, id: VendorId) -> Result<Option<Vendor>, AppError>;

    async fn find_all(&self) -> Result<Vec<Vendor>, AppError>;

    async fn delete(&self, id: VendorId) -> Result<(), AppError>;
}
