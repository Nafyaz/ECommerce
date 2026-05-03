use crate::modules::vendors::application::command_results::CreateVendorResult;
use crate::modules::vendors::application::commands::CreateVendorCommand;
use crate::modules::vendors::errors::VendorDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait VendorCommandPort: Send + Sync {
    async fn create_vendor(&self, command: CreateVendorCommand) -> Result<CreateVendorResult, VendorDomainError>;
}
