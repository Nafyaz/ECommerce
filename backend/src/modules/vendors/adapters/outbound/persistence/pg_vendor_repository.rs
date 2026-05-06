use crate::modules::vendors::adapters::outbound::persistence::vendor_record::VendorRecord;
use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::domain::value_objects::VendorId;
use crate::modules::vendors::errors::VendorDomainError;
use crate::modules::vendors::ports::outbound::VendorRepositoryPort;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgVendorRepository {
    pool: PgPool,
}

impl PgVendorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for VendorDomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => VendorDomainError::VendorNotFound,
            _ => {
                tracing::error!("Database error: {:?}", err);
                VendorDomainError::InternalError("An internal database error occurred".to_string())
            }
        }
    }
}

#[async_trait]
impl VendorRepositoryPort for PgVendorRepository {
    async fn save(&self, vendor: &Vendor) -> Result<(), VendorDomainError> {
        let row = VendorRecord::from_entity(vendor);

        sqlx::query(
            "INSERT INTO vendors \
            (id, name, owner_id, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.id)
        .bind(&row.name)
        .bind(row.owner_id)
        .bind(row.created_at)
        .bind(row.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &VendorId) -> Result<Option<Vendor>, VendorDomainError> {
        let row = sqlx::query_as::<_, VendorRecord>(
            "SELECT id, name, owner_id, created_at, updated_at FROM vendors WHERE id = $1",
        )
        .bind(id.as_uuid().to_owned())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(VendorRecord::into_entity))
    }

    async fn find_all(&self) -> Result<Vec<Vendor>, VendorDomainError> {
        todo!()
    }

    async fn delete(&self, id: &VendorId) -> Result<(), VendorDomainError> {
        todo!()
    }
}
