use crate::modules::product::application::ProductAppError;
use crate::modules::product::application::commands::CreateProductCommand;
use crate::modules::product::application::results::CreateProductResult;
use async_trait::async_trait;

#[async_trait]
pub trait ProductCommandPort: Send + Sync {
    async fn create_product(&self, product: CreateProductCommand) -> Result<CreateProductResult, ProductAppError>;
}
