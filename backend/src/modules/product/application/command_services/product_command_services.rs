use crate::modules::product::application::command_results::CreateProductResult;
use crate::modules::product::application::commands::CreateProductCommand;
use crate::modules::product::domain::entities::Product;
use crate::modules::product::errors::ProductDomainError;
use crate::modules::product::ports::inbound::ProductCommandPort;
use crate::modules::product::ports::outbound::{
    ProductIdentityPort, ProductIdentityPortError, ProductRepositoryPort, ProductVendorPort, ProductVendorPortError,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductCommandService {
    product_identity_provider: Arc<dyn ProductIdentityPort>,
    product_vendor_provider: Arc<dyn ProductVendorPort>,
    product_repo: Arc<dyn ProductRepositoryPort>,
}

impl ProductCommandService {
    pub fn new(
        product_identity_provider: Arc<dyn ProductIdentityPort>,
        product_vendor_provider: Arc<dyn ProductVendorPort>,
        product_repo: Arc<dyn ProductRepositoryPort>,
    ) -> Self {
        Self {
            product_identity_provider,
            product_vendor_provider,
            product_repo,
        }
    }
}

#[async_trait]
impl ProductCommandPort for ProductCommandService {
    async fn create_product(&self, command: CreateProductCommand) -> Result<CreateProductResult, ProductDomainError> {
        let is_verified = self
            .product_identity_provider
            .check_verified(command.current_actor_id())
            .await
            .map_err(|error| match error {
                ProductIdentityPortError::NotFound => {
                    ProductDomainError::ActorNotVerified(command.current_actor_id().as_uuid().to_owned())
                }
                ProductIdentityPortError::Unavailable | ProductIdentityPortError::Unexpected => {
                    ProductDomainError::IdentityPortError
                }
            })?;

        if !is_verified {
            return Err(ProductDomainError::ActorNotVerified(
                command.current_actor_id().as_uuid().to_owned(),
            ));
        }

        let owns_supplier = self
            .product_vendor_provider
            .check_ownership(command.supplier_id(), command.current_actor_id())
            .await
            .map_err(|error| match error {
                ProductVendorPortError::NotFound => ProductDomainError::VendorNotFound,
                ProductVendorPortError::Unavailable | ProductVendorPortError::Unexpected => {
                    ProductDomainError::VendorPortError
                }
            })?;

        if !owns_supplier {
            return Err(ProductDomainError::VendorOwnershipMismatch);
        }

        let product = Product::new(
            command.name().to_owned(),
            command.supplier_id().to_owned(),
            command.price().to_owned(),
        )?;

        self.product_repo.save(&product).await?;

        tracing::info!("Product created: {}", product.id().as_uuid());

        Ok(CreateProductResult::new(product.id().as_uuid().to_owned()))
    }
}
