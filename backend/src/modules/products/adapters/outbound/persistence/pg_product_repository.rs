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
        todo!()
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
