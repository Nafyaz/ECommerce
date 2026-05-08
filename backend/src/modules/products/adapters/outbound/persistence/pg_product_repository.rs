use crate::modules::products::adapters::outbound::persistence::product_record::ProductRecord;
use crate::modules::products::domain::entities::Product;
use crate::modules::products::domain::value_objects::ProductId;
use crate::modules::products::errors::ProductDomainError;
use crate::modules::products::ports::outbound::ProductRepositoryPort;
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

impl From<sqlx::Error> for ProductDomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => ProductDomainError::VendorNotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                ProductDomainError::InternalError("An internal database error occurred".to_string())
            }
        }
    }
}

#[async_trait]
impl ProductRepositoryPort for PgProductRepository {
    async fn save(&self, product: &Product) -> Result<(), ProductDomainError> {
        let record = ProductRecord::from_entity(product);

        sqlx::query(
            "INSERT INTO products \
            (id, name, supplier_id, price_amount_minor, price_currency, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
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

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, ProductDomainError> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<Product>, ProductDomainError> {
        todo!()
    }

    async fn delete(&self, id: &ProductId) -> Result<(), ProductDomainError> {
        todo!()
    }
}
