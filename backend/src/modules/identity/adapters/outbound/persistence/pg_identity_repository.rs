use crate::modules::identity::IdentityError;
use crate::modules::identity::adapters::outbound::persistence::IdentityRecord;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{Email, IdentityId, IdentityStatus};
use crate::modules::identity::ports::outbound::IdentityRepositoryPort;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgIdentityRepository {
    pool: PgPool,
}

impl PgIdentityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IdentityRepositoryPort for PgIdentityRepository {
    async fn save(&self, identity: &Identity) -> Result<(), IdentityError> {
        let record = IdentityRecord::from_entity(identity);

        sqlx::query(
            "INSERT INTO identities \
            (id, email, password_hash, status, created_at, updated_at) \
            VALUES ($1, $2, $3, $4::identity_status, $5, $6)",
        )
        .bind(record.id())
        .bind(record.email())
        .bind(record.password_hash())
        .bind(record.status())
        .bind(record.created_at())
        .bind(record.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update(&self, identity: &Identity) -> Result<(), IdentityError> {
        let record = IdentityRecord::from_entity(identity);

        sqlx::query(
            "UPDATE identities \
            SET email = $2, password_hash = $3, status = $4::identity_status, updated_at = $5 \
            WHERE id = $1",
        )
        .bind(record.id())
        .bind(record.email())
        .bind(record.password_hash())
        .bind(record.status())
        .bind(record.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &IdentityId) -> Result<Option<Identity>, IdentityError> {
        let record = sqlx::query_as::<_, IdentityRecord>(
            "SELECT id, email, password_hash, status::TEXT, created_at, updated_at \
            FROM identities \
            WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(Identity::try_from).transpose()?)
    }

    async fn find_verified_by_email(&self, email: &Email) -> Result<Option<Identity>, IdentityError> {
        let email = email.as_str();
        let record = sqlx::query_as::<_, IdentityRecord>(
            "SELECT id, email, password_hash, status::TEXT, created_at, updated_at \
            FROM identities \
            WHERE email = $1 AND status = $2::identity_status",
        )
        .bind(email)
        .bind(IdentityStatus::Verified.as_str())
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(Identity::try_from).transpose()?)
    }

    async fn find_all(&self) -> Result<Vec<Identity>, IdentityError> {
        todo!()
    }

    async fn find_by_role(&self, role: &str) -> Result<Vec<Identity>, IdentityError> {
        todo!()
    }

    async fn delete(&self, id: &IdentityId) -> Result<(), IdentityError> {
        todo!()
    }
}
