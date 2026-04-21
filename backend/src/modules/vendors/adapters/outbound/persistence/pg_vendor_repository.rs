use crate::modules::shared::AppError;
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
    async fn save(&self, vendor: &Vendor) -> Result<(), AppError> {}

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
