use crate::modules::product::application::ProductImageAppError;
use crate::modules::product::domain::value_objects::ProductImageId;
use async_trait::async_trait;

#[async_trait]
pub trait ProductImageQueryPort {
    async fn get_product_image(&self, product_image_id: ProductImageId) -> Result<(), ProductImageAppError>;
}
