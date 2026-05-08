use crate::modules::product::application::command_results::CreateProductResult;
use crate::modules::product::application::commands::CreateProductCommand;
use crate::modules::product::errors::ProductDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait ProductCommandPort: Send + Sync {
    async fn create_product(&self, product: CreateProductCommand) -> Result<CreateProductResult, ProductDomainError>;
}
