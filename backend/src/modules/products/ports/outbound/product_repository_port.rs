use crate::modules::products::domain::entities::Product;
use crate::modules::products::domain::value_objects::ProductId;
use crate::modules::products::errors::ProductDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepositoryPort: Send + Sync {
    async fn save(&self, product: &Product) -> Result<(), ProductDomainError>;
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductDomainError>;
    async fn find_all(&self) -> Result<Vec<Product>, ProductDomainError>;
    async fn delete(&self, id: &ProductId) -> Result<(), ProductDomainError>;
}
