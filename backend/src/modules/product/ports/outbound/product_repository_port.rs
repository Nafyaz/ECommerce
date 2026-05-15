use crate::modules::product::domain::entities::Product;
use crate::modules::product::domain::value_objects::ProductId;
use crate::modules::product::ports::outbound::product_repository_error::ProductRepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepositoryPort: Send + Sync {
    async fn save(&self, product: &Product) -> Result<(), ProductRepositoryError>;
    async fn find_by_id(&self, id: ProductId) -> Result<Option<Product>, ProductRepositoryError>;
    async fn find_all(&self) -> Result<Vec<Product>, ProductRepositoryError>;
    async fn delete(&self, id: ProductId) -> Result<(), ProductRepositoryError>;
}
