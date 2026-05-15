use crate::modules::product::application::ProductAppError;
use crate::modules::product::application::commands::CreateProductCommand;
use crate::modules::product::application::results::CreateProductResult;
use crate::modules::product::domain::entities::Product;
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
    async fn create_product(&self, command: CreateProductCommand) -> Result<CreateProductResult, ProductAppError> {
        let is_verified = self
            .product_identity_provider
            .check_verified(command.current_actor_id())
            .await
            .map_err(|error| match error {
                ProductIdentityPortError::NotFound => ProductAppError::ActorNotVerified,
                ProductIdentityPortError::Unavailable => ProductAppError::DependencyUnavailable("identity"),
                ProductIdentityPortError::Unexpected => ProductAppError::Internal,
            })?;

        if !is_verified {
            return Err(ProductAppError::ActorNotVerified);
        }

        let owns_supplier = self
            .product_vendor_provider
            .check_ownership(command.supplier_id(), command.current_actor_id())
            .await
            .map_err(|error| match error {
                ProductVendorPortError::NotFound => ProductAppError::VendorNotFound,
                ProductVendorPortError::Unavailable => ProductAppError::DependencyUnavailable("vendor"),
                ProductVendorPortError::Unexpected => ProductAppError::Internal,
            })?;

        if !owns_supplier {
            return Err(ProductAppError::VendorOwnershipMismatch);
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
