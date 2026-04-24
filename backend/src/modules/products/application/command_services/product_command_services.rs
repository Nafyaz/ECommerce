use crate::modules::products::application::command_results::CreateProductResult;
use crate::modules::products::application::commands::CreateProductCommand;
use crate::modules::products::domain::entities::Product;
use crate::modules::products::errors::ProductDomainError;
use crate::modules::products::ports::inbound::ProductCommandPort;
use crate::modules::products::ports::outbound::ProductRepositoryPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductCommandService {
    product_repo: Arc<dyn ProductRepositoryPort>,
}

impl ProductCommandService {
    pub fn new(product_repo: Arc<dyn ProductRepositoryPort>) -> Self {
        Self { product_repo }
    }
}

#[async_trait]
impl ProductCommandPort for ProductCommandService {
    async fn create_product(&self, command: CreateProductCommand) -> Result<CreateProductResult, ProductDomainError> {
        let product = Product::new(
            command.name().to_owned(),
            command.vendor_id().to_owned(),
            command.price().to_owned(),
        )?;

        self.product_repo.save(&product).await?;

        tracing::info!("Product created: {}", product.id().as_uuid());

        let result = CreateProductResult::new(product.id().as_uuid().to_owned());

        Ok(result)
    }
}
