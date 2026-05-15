use crate::modules::product::application::ProductImageAppError;
use crate::modules::product::application::commands::{ConfirmUploadCommand, CreateUploadCommand};
use crate::modules::product::application::results::CreateUploadResult;
use async_trait::async_trait;

#[async_trait]
pub trait ProductImageCommandPort: Send + Sync {
    async fn create_upload(&self, command: CreateUploadCommand) -> Result<CreateUploadResult, ProductImageAppError>;
    async fn confirm_product_image_upload(&self, command: ConfirmUploadCommand) -> Result<(), ProductImageAppError>;
}
