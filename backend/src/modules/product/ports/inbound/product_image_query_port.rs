use crate::modules::product::domain::value_objects::ProductImageId;
use crate::modules::product::errors::ImageError;
use async_trait::async_trait;

#[async_trait]
pub trait ProductImageQueryPort {
    async fn get_product_image(&self, product_image_id: ProductImageId) -> Result<(), ImageError>;
}
