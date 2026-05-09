use async_trait::async_trait;

use crate::modules::product::domain::entities::ProductImage;
use crate::modules::product::domain::value_objects::{ProductId, ProductImageId};
use crate::modules::product::errors::ImageError;

#[async_trait]
pub trait ProductImageRepositoryPort: Send + Sync {
    async fn save(&self, image: &ProductImage) -> Result<(), ImageError>;

    async fn update(&self, image: &ProductImage) -> Result<(), ImageError>;

    async fn find_by_id(&self, id: ProductImageId) -> Result<Option<ProductImage>, ImageError>;

    async fn find_by_product_id(&self, product_id: ProductId) -> Result<Vec<ProductImage>, ImageError>;

    async fn delete(&self, id: ProductImageId) -> Result<(), ImageError>;
}
