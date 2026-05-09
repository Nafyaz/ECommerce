use crate::modules::product::domain::entities::ProductImage;
use crate::modules::product::domain::value_objects::{
    ContentType, FileSize, ObjectKey, ProductId, ProductImageId, ProductImageStatus,
};
use crate::modules::product::errors::ImageError;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ProductImageRecord {
    id: Uuid,
    product_id: Uuid,
    object_key: String,
    content_type: String,
    status: String,
    file_size: i64,
    display_order: i16,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProductImageRecord {
    pub fn from_entity(product_image: &ProductImage) -> Self {
        Self {
            id: product_image.id().as_uuid(),
            product_id: product_image.product_id().as_uuid(),
            object_key: product_image.object_key().as_str().to_owned(),
            content_type: product_image.content_type().as_str().to_owned(),
            status: product_image.status().as_str().to_owned(),
            file_size: product_image.file_size().as_i64(),
            display_order: product_image.display_order() as i16,
            created_at: product_image.created_at(),
            updated_at: product_image.updated_at(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn product_id(&self) -> Uuid {
        self.product_id
    }

    pub fn object_key(&self) -> &str {
        &self.object_key
    }
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn file_size(&self) -> i64 {
        self.file_size
    }

    pub fn display_order(&self) -> i16 {
        self.display_order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl TryFrom<ProductImageRecord> for ProductImage {
    type Error = ImageError;

    fn try_from(product_image_record: ProductImageRecord) -> Result<Self, Self::Error> {
        ProductImage::reconstitute(
            ProductImageId::from_uuid(product_image_record.id),
            ProductId::from_uuid(product_image_record.product_id),
            ObjectKey::from_str(product_image_record.object_key)?,
            ContentType::from_str(product_image_record.content_type)?,
            ProductImageStatus::from_str(product_image_record.status)?,
            FileSize::from_i64(product_image_record.file_size)?,
            product_image_record.display_order as u8,
            product_image_record.created_at,
            product_image_record.updated_at,
        )
    }
}
