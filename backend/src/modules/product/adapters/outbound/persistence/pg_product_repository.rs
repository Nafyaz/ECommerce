use crate::modules::product::adapters::outbound::persistence::product_record::ProductRecord;
use crate::modules::product::domain::entities::Product;
use crate::modules::product::domain::value_objects::ProductId;
use crate::modules::product::errors::ProductError;
use crate::modules::product::ports::outbound::ProductRepositoryPort;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgProductRepository {
    pool: PgPool,
}

impl PgProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for ProductError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ProductError::VendorNotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                ProductError::InternalError("An internal database error occurred".to_string())
            }
        }
    }
}

#[async_trait]
impl ProductRepositoryPort for PgProductRepository {
    async fn save(&self, product: &Product) -> Result<(), ProductError> {
        let record = ProductRecord::from_entity(product);

        sqlx::query(
            "INSERT INTO products \
            (id, name, supplier_id, price_amount_minor, price_currency, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5::currency, $6, $7)",
        )
        .bind(record.id())
        .bind(record.name())
        .bind(record.supplier_id())
        .bind(record.price_amount_minor())
        .bind(record.price_currency())
        .bind(record.created_at())
        .bind(record.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: ProductId) -> Result<Option<Product>, ProductError> {
        let row = sqlx::query_as::<_, ProductRecord>(
            "SELECT id, name, supplier_id, price_amount_minor, price_currency::TEXT, created_at, updated_at \
            FROM products \
            WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Product::try_from).transpose()?)
    }

    async fn find_all(&self) -> Result<Vec<Product>, ProductError> {
        todo!()
    }

    async fn delete(&self, id: ProductId) -> Result<(), ProductError> {
        todo!()
    }
}
