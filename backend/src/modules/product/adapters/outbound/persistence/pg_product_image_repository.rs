use crate::modules::product::adapters::outbound::persistence::product_image_record::ProductImageRecord;
use crate::modules::product::domain::entities::ProductImage;
use crate::modules::product::domain::value_objects::{ProductId, ProductImageId};
use crate::modules::product::errors::ImageError;
use crate::modules::product::ports::outbound::ProductImageRepositoryPort;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgProductImageRepository {
    pool: PgPool,
}

impl PgProductImageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for ImageError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        ImageError::NotFound
    }
}

#[async_trait]
impl ProductImageRepositoryPort for PgProductImageRepository {
    async fn save(&self, image: &ProductImage) -> Result<(), ImageError> {
        let record = ProductImageRecord::from_entity(image);

        sqlx::query(
            "INSERT INTO product_images \
            (id, product_id, object_key, content_type, status, file_size, display_order, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5::product_image_status, $6, $7, $8, $9)",
        )
        .bind(record.id())
        .bind(record.product_id())
        .bind(record.object_key())
        .bind(record.content_type())
        .bind(record.status())
        .bind(record.file_size())
        .bind(record.display_order())
        .bind(record.created_at())
        .bind(record.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, product_image: &ProductImage) -> Result<(), ImageError> {
        let row = ProductImageRecord::from_entity(product_image);

        sqlx::query(
            "UPDATE product_images \
            SET status = $2::product_image_status, display_order = $3, updated_at = $4 \
            WHERE id = $1",
        )
        .bind(row.id())
        .bind(row.status())
        .bind(row.display_order())
        .bind(row.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: ProductImageId) -> Result<Option<ProductImage>, ImageError> {
        let row = sqlx::query_as::<_, ProductImageRecord>(
            "SELECT id, product_id, object_key, content_type, status::TEXT, file_size, display_order, created_at, updated_at \
            FROM product_images \
            WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(ProductImage::try_from).transpose()?)
    }

    async fn find_by_product_id(&self, product_id: ProductId) -> Result<Vec<ProductImage>, ImageError> {
        let rows = sqlx::query_as::<_, ProductImageRecord>(
            "
            SELECT id, product_id, object_key, content_type, status::TEXT, file_size, display_order, created_at, updated_at
            FROM product_images
            WHERE product_id = $1
            ORDER BY display_order, created_at
            ",
        )
        .bind(product_id.as_uuid())
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(ProductImage::try_from).collect()
    }

    async fn delete(&self, id: ProductImageId) -> Result<(), ImageError> {
        sqlx::query("DELETE FROM product_images WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
