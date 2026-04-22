use crate::modules::shared::AppError;
use crate::modules::vendors::application::command_results::CreateVendorResult;
use crate::modules::vendors::application::commands::CreateVendorCommand;
use crate::modules::vendors::domain::value_objects::OwnerId;
use async_trait::async_trait;

#[async_trait]
pub trait VendorCommandPort: Send + Sync {
    async fn create_vendor(
        &self,
        command: CreateVendorCommand,
        owner_id: OwnerId,
    ) -> Result<CreateVendorResult, AppError>;
}
