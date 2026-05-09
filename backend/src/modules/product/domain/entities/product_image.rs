use crate::modules::product::domain::value_objects::{
    ContentType, FileSize, ObjectKey, ProductId, ProductImageId, ProductImageStatus,
};
use crate::modules::product::errors::ImageError;
use chrono::{DateTime, Utc};

pub struct ProductImage {
    id: ProductImageId,
    product_id: ProductId,
    object_key: ObjectKey,
    content_type: ContentType,
    status: ProductImageStatus,
    file_size: FileSize,
    display_order: u8,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProductImage {
    pub fn new(
        product_id: ProductId,
        content_type: ContentType,
        file_size: FileSize,
        display_order: u8,
    ) -> Result<Self, ImageError> {
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
        display_order: u8,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, ImageError> {
        if updated_at < created_at {
            return Err(ImageError::InvalidTimestamps);
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

    pub fn confirm_upload(&mut self) {
        self.status = ProductImageStatus::Uploaded;
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

    pub fn display_order(&self) -> u8 {
        self.display_order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
