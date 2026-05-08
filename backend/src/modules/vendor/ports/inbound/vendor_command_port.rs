use crate::modules::vendor::application::command_results::CreateVendorResult;
use crate::modules::vendor::application::commands::CreateVendorCommand;
use crate::modules::vendor::errors::VendorDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait VendorCommandPort: Send + Sync {
    async fn create_vendor(&self, command: CreateVendorCommand) -> Result<CreateVendorResult, VendorDomainError>;
}
