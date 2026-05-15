use crate::modules::product::domain::ProductImageDomainError;
use crate::modules::product::domain::value_objects::{
    ContentType, FileSize, ObjectKey, ProductId, ProductImageId, ProductImageStatus,
};
use chrono::{DateTime, Utc};

pub struct ProductImage {
    id: ProductImageId,
    product_id: ProductId,
    object_key: ObjectKey,
    content_type: ContentType,
    status: ProductImageStatus,
    file_size: FileSize,
    display_order: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProductImage {
    pub fn new(
        product_id: ProductId,
        content_type: ContentType,
        file_size: FileSize,
        display_order: i32,
    ) -> Result<Self, ProductImageDomainError> {
        if display_order < 0 {
            return Err(ProductImageDomainError::InvalidDisplayOrder(display_order));
        }

        let product_image_id = ProductImageId::new();
        let now = Utc::now();

        Ok(Self {
            id: product_image_id,
            product_id,
            object_key: ObjectKey::new(product_id, product_image_id),
            content_type,
            status: ProductImageStatus::PendingUpload,
            file_size,
            display_order,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: ProductImageId,
        product_id: ProductId,
        object_key: ObjectKey,
        content_type: ContentType,
        status: ProductImageStatus,
        file_size: FileSize,
        display_order: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, ProductImageDomainError> {
        if display_order < 0 {
            return Err(ProductImageDomainError::InvalidDisplayOrder(display_order));
        }

        if updated_at < created_at {
            return Err(ProductImageDomainError::InvalidTimestamps(
                "Product Image updated_at cannot be earlier than created_at".to_owned(),
            ));
        }

        Ok(Self {
            id,
            product_id,
            object_key,
            content_type,
            status,
            file_size,
            display_order,
            created_at,
            updated_at,
        })
    }

    pub fn confirm_upload(&mut self) -> Result<(), ProductImageDomainError> {
        if self.status != ProductImageStatus::PendingUpload {
            return Err(ProductImageDomainError::InvalidStateTransition);
        }

        self.status = ProductImageStatus::Uploaded;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn id(&self) -> ProductImageId {
        self.id
    }

    pub fn product_id(&self) -> ProductId {
        self.product_id
    }

    pub fn object_key(&self) -> &ObjectKey {
        &self.object_key
    }

    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    pub fn file_size(&self) -> FileSize {
        self.file_size
    }

    pub fn status(&self) -> ProductImageStatus {
        self.status
    }

    pub fn display_order(&self) -> i32 {
        self.display_order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
