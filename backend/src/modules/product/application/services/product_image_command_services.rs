use crate::modules::product::application::commands::{ConfirmUploadCommand, CreateUploadCommand};
use crate::modules::product::application::results::CreateUploadResult;
use crate::modules::product::domain::entities::ProductImage;
use crate::modules::product::domain::value_objects::ProductImageStatus;
use crate::modules::product::errors::ImageError;
use crate::modules::product::ports::inbound::ProductImageCommandPort;
use crate::modules::product::ports::outbound::{
    ObjectStoragePort, ProductIdentityPort, ProductImageRepositoryPort, ProductRepositoryPort, ProductVendorPort,
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
    async fn create_upload(&self, command: CreateUploadCommand) -> Result<CreateUploadResult, ImageError> {
        let is_verified = self
            .product_identity_provider
            .check_verified(command.current_actor_id())
            .await
            .map_err(|error| ImageError::NotFound)?;

        if !is_verified {
            return Err(ImageError::NotFound);
        }

        let product = self
            .product_repo
            .find_by_id(command.product_id())
            .await
            .map_err(|error| ImageError::NotFound)?
            .ok_or(ImageError::NotFound)?;

        let owns_supplier = self
            .product_vendor_provider
            .check_ownership(product.supplier_id(), command.current_actor_id())
            .await
            .map_err(|error| ImageError::NotFound)?;

        if !owns_supplier {
            return Err(ImageError::NotFound);
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
            .await
            .map_err(|_| ImageError::NotFound)?;

        let result = CreateUploadResult::new(
            product_image.id().as_uuid(),
            presigned_upload.upload_url,
            presigned_upload.expires_at,
        );

        Ok(result)
    }

    // TODO: need to ensure actor is verified and supplier's owner and this image belongs to this product.
    async fn confirm_product_image_upload(&self, command: ConfirmUploadCommand) -> Result<(), ImageError> {
        let image_id = command.image_id();

        tracing::debug!("Confirming product image upload for image ID: {}", image_id);

        let mut image = self
            .product_image_repo
            .find_by_id(image_id)
            .await?
            .ok_or(ImageError::NotFound)?;

        tracing::debug!("{}", image.status().as_str());

        if image.status() != ProductImageStatus::PendingUpload {
            return Err(ImageError::InvalidState);
        }

        tracing::debug!("Product image status is PendingUpload");

        let exists = self
            .object_storage
            .check_object_exists(&image.object_key())
            .await
            .map_err(|_| ImageError::NotFound)?;

        if !exists {
            return Err(ImageError::NotFound);
        }

        tracing::debug!("Product image exists in object storage");

        image.confirm_upload();
        self.product_image_repo.update(&image).await?;

        tracing::debug!("Product image confirmed");

        Ok(())
    }
}
