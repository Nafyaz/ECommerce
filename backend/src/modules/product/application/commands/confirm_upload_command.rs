use crate::modules::product::application::ProductImageAppError;
use crate::modules::product::domain::value_objects::ProductImageId;
use uuid::Uuid;

pub struct ConfirmUploadCommand {
    image_id: ProductImageId,
}

impl ConfirmUploadCommand {
    pub fn new(image_id: Uuid) -> Result<Self, ProductImageAppError> {
        let image_id = ProductImageId::from_uuid(image_id);

        Ok(Self { image_id })
    }

    pub fn image_id(&self) -> ProductImageId {
        self.image_id
    }
}
