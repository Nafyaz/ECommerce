use crate::modules::product::application::ProductImageAppError;
use crate::modules::product::domain::value_objects::ProductImageId;
use crate::modules::product::ports::inbound::ProductImageQueryPort;
use crate::modules::product::ports::outbound::{ObjectStoragePort, ProductImageRepositoryPort, ProductRepositoryPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductImageQueryService {
    product_repo: Arc<dyn ProductRepositoryPort>,
    product_image_repo: Arc<dyn ProductImageRepositoryPort>,
    object_storage: Arc<dyn ObjectStoragePort>,
}

impl ProductImageQueryService {
    pub fn new(
        product_repo: Arc<dyn ProductRepositoryPort>,
        product_image_repo: Arc<dyn ProductImageRepositoryPort>,
        object_storage: Arc<dyn ObjectStoragePort>,
    ) -> Self {
        Self {
            product_repo,
            product_image_repo,
            object_storage,
        }
    }
}

#[async_trait]
impl ProductImageQueryPort for ProductImageQueryService {
    async fn get_product_image(&self, _product_image_id: ProductImageId) -> Result<(), ProductImageAppError> {
        todo!()
    }
}
