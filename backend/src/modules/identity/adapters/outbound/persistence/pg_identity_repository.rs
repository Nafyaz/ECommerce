use crate::modules::identity::IdentityError;
use crate::modules::identity::adapters::outbound::persistence::IdentityRow;
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
        let row = IdentityRow::from_entity(identity);

        sqlx::query(
            "INSERT INTO identities \
            (id, email, password_hash, status, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(row.id())
        .bind(row.email())
        .bind(row.password_hash())
        .bind(row.status())
        .bind(row.created_at())
        .bind(row.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &IdentityId) -> Result<Option<Identity>, IdentityError> {
        let row = sqlx::query_as::<_, IdentityRow>(
            "SELECT id, email, password_hash, status, created_at, updated_at \
            FROM identities \
            WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Identity::try_from).transpose()?)
    }

    async fn find_verified_by_email(&self, email: &Email) -> Result<Option<Identity>, IdentityError> {
        let email = email.as_str();
        let row = sqlx::query_as::<_, IdentityRow>(
            "SELECT id, email, password_hash, status, created_at, updated_at \
            FROM identities \
            WHERE email = $1 AND status = $2",
        )
        .bind(email)
        .bind(IdentityStatus::Verified.as_str())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(IdentityRow::into_entity))
    }

    async fn mark_email_verified(&self, identity: &Identity) -> Result<(), IdentityError> {
        sqlx::query(
            "UPDATE identities \
            SET email_verified_at = $1, updated_at = $2 \
            WHERE id = $3",
        )
        .bind(identity.email_verified_at())
        .bind(identity.updated_at())
        .bind(identity.id().as_uuid())
        .execute(&self.pool)
        .await?;

        Ok(())
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
