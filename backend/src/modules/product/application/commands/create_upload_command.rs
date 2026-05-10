use crate::modules::product::domain::value_objects::{ContentType, FileSize, ProductActorId, ProductId};
use crate::modules::product::errors::ImageError;
use uuid::Uuid;

// TODO: db and application check for same product, same image, same display_order
pub struct CreateUploadCommand {
    current_actor_id: ProductActorId,
    product_id: ProductId,
    content_type: ContentType,
    file_size: FileSize,
    display_order: i32,
}

impl CreateUploadCommand {
    pub fn new(
        current_actor_id: Uuid,
        product_id: Uuid,
        content_type: String,
        file_size: i64,
        display_order: i32,
    ) -> Result<Self, ImageError> {
        let current_actor_id = ProductActorId::from_uuid(current_actor_id);
        let product_id = ProductId::from_uuid(product_id);
        let content_type = ContentType::from_str(content_type)?;
        let file_size = FileSize::from_i64(file_size)?;

        Ok(Self {
            current_actor_id,
            product_id,
            content_type,
            file_size,
            display_order,
        })
    }

    pub fn current_actor_id(&self) -> ProductActorId {
        self.current_actor_id
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    pub fn file_size(&self) -> FileSize {
        self.file_size
    }

    pub fn display_order(&self) -> i32 {
        self.display_order
    }
}
