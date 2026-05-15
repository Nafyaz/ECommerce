use crate::modules::product::application::ProductImageAppError;
use crate::modules::product::application::commands::{ConfirmUploadCommand, CreateUploadCommand};
use crate::modules::product::application::results::CreateUploadResult;
use crate::modules::product::domain::entities::ProductImage;
use crate::modules::product::ports::inbound::ProductImageCommandPort;
use crate::modules::product::ports::outbound::{
    ObjectStoragePort, ProductIdentityPort, ProductIdentityPortError, ProductImageRepositoryPort,
    ProductRepositoryPort, ProductVendorPort, ProductVendorPortError,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductImageCommandService {
    product_identity_provider: Arc<dyn ProductIdentityPort>,
    product_vendor_provider: Arc<dyn ProductVendorPort>,
    product_repo: Arc<dyn ProductRepositoryPort>,
    product_image_repo: Arc<dyn ProductImageRepositoryPort>,
    object_storage: Arc<dyn ObjectStoragePort>,
}

impl ProductImageCommandService {
    pub fn new(
        product_identity_provider: Arc<dyn ProductIdentityPort>,
        product_vendor_provider: Arc<dyn ProductVendorPort>,
        product_repo: Arc<dyn ProductRepositoryPort>,
        product_image_repo: Arc<dyn ProductImageRepositoryPort>,
        object_storage: Arc<dyn ObjectStoragePort>,
    ) -> Self {
        Self {
            product_identity_provider,
            product_vendor_provider,
            product_repo,
            product_image_repo,
            object_storage,
        }
    }
}

#[async_trait]
impl ProductImageCommandPort for ProductImageCommandService {
    async fn create_upload(&self, command: CreateUploadCommand) -> Result<CreateUploadResult, ProductImageAppError> {
        let is_verified = self
            .product_identity_provider
            .check_verified(command.current_actor_id())
            .await
            .map_err(|error| match error {
                ProductIdentityPortError::NotFound => ProductImageAppError::ActorNotVerified,
                ProductIdentityPortError::Unavailable => ProductImageAppError::DependencyUnavailable("identity"),
                ProductIdentityPortError::Unexpected => ProductImageAppError::Internal,
            })?;

        if !is_verified {
            return Err(ProductImageAppError::ActorNotVerified);
        }

        let product = self
            .product_repo
            .find_by_id(command.product_id())
            .await
            .map_err(ProductImageAppError::from_product_repository_error)?
            .ok_or(ProductImageAppError::ProductNotFound)?;

        let owns_supplier = self
            .product_vendor_provider
            .check_ownership(product.supplier_id(), command.current_actor_id())
            .await
            .map_err(|error| match error {
                ProductVendorPortError::NotFound => ProductImageAppError::Internal,
                ProductVendorPortError::Unavailable => ProductImageAppError::DependencyUnavailable("vendor"),
                ProductVendorPortError::Unexpected => ProductImageAppError::Internal,
            })?;

        if !owns_supplier {
            return Err(ProductImageAppError::VendorOwnershipMismatch);
        }

        let product_image = ProductImage::new(
            product.id(),
            command.content_type().to_owned(),
            command.file_size(),
            command.display_order(),
        )?;

        self.product_image_repo.save(&product_image).await?;

        let presigned_upload = self
            .object_storage
            .generate_presigned_url(&product_image.object_key(), command.content_type(), command.file_size())
            .await?;

        let result = CreateUploadResult::new(
            product_image.id().as_uuid(),
            presigned_upload.upload_url,
            presigned_upload.expires_at,
        );

        Ok(result)
    }

    // TODO: need to ensure actor is verified and supplier's owner and this image belongs to this product.
    async fn confirm_product_image_upload(&self, command: ConfirmUploadCommand) -> Result<(), ProductImageAppError> {
        let image_id = command.image_id();

        tracing::debug!("Confirming product image upload for image ID: {}", image_id);

        let mut image = self
            .product_image_repo
            .find_by_id(image_id)
            .await?
            .ok_or(ProductImageAppError::ImageNotFound)?;

        tracing::debug!("{}", image.status().as_str());

        let exists = self.object_storage.check_object_exists(&image.object_key()).await?;

        if !exists {
            return Err(ProductImageAppError::ImageNotFound);
        }

        tracing::debug!("Product image exists in object storage");

        image.confirm_upload()?;
        self.product_image_repo.update(&image).await?;

        tracing::debug!("Product image confirmed");

        Ok(())
    }
}
