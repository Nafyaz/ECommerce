use crate::modules::product::application::command_results::CreateUploadResult;
use crate::modules::product::application::commands::{ConfirmUploadCommand, CreateUploadCommand};
use crate::modules::product::errors::ImageError;
use async_trait::async_trait;

#[async_trait]
pub trait ProductImageCommandPort: Send + Sync {
    async fn create_upload(&self, command: CreateUploadCommand) -> Result<CreateUploadResult, ImageError>;
    async fn confirm_product_image_upload(&self, command: ConfirmUploadCommand) -> Result<(), ImageError>;
}
