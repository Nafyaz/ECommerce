// use crate::modules::products::application::command_results::CreateProductResult;
// use crate::modules::products::application::commands::CreateProductCommand;
// use crate::modules::products::domain::entities::Product;
// use crate::modules::products::errors::ProductDomainError;
// use crate::modules::products::ports::inbound::ProductCommandPort;
// use crate::modules::products::ports::outbound::ProductRepositoryPort;
// use crate::modules::vendors::{OwnerId, VendorId, VendorRepositoryPort};
// use async_trait::async_trait;
// use std::sync::Arc;
//
// pub struct ProductCommandService {
//     product_repo: Arc<dyn ProductRepositoryPort>,
//     vendor_repo: Arc<dyn VendorRepositoryPort>,
// }
//
// impl ProductCommandService {
//     pub fn new(product_repo: Arc<dyn ProductRepositoryPort>, vendor_repo: Arc<dyn VendorRepositoryPort>) -> Self {
//         Self {
//             product_repo,
//             vendor_repo,
//         }
//     }
// }
//
// #[async_trait]
// impl ProductCommandPort for ProductCommandService {
//     async fn create_product(&self, command: CreateProductCommand) -> Result<CreateProductResult, ProductDomainError> {
//         let vendor_id = VendorId::from_uuid(*command.vendor_id().as_uuid());
//         let vendor = self
//             .vendor_repo
//             .find_by_id(vendor_id)
//             .await
//             .map_err(ProductDomainError::from)?
//             .ok_or(ProductDomainError::VendorNotFound)?;
//
//         let current_owner_id = OwnerId::from_uuid(command.current_user_id());
//         if vendor.owner_id() != &current_owner_id {
//             return Err(ProductDomainError::VendorOwnershipMismatch);
//         }
//
//         let product = Product::new(
//             command.name().to_owned(),
//             command.vendor_id().to_owned(),
//             command.price().to_owned(),
//         )?;
//
//         self.product_repo.save(&product).await?;
//
//         tracing::info!("Product created: {}", product.id().as_uuid());
//
//         let result = CreateProductResult::new(*product.id().as_uuid());
//
//         Ok(result)
//     }
// }
