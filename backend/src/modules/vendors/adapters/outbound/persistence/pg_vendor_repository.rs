use crate::modules::shared::AppError;
use crate::modules::vendors::adapters::outbound::persistence::mapper::VendorRow;
use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::domain::value_objects::VendorId;
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

#[async_trait]
impl VendorRepositoryPort for PgVendorRepository {
    async fn save(&self, vendor: &Vendor) -> Result<(), AppError> {
        let row = VendorRow::from_entity(vendor);

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

    async fn find_by_id(&self, id: VendorId) -> Result<Option<Vendor>, AppError> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<Vendor>, AppError> {
        todo!()
    }

    async fn delete(&self, id: VendorId) -> Result<(), AppError> {
        todo!()
    }
}
