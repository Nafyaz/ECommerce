use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::adapters::outbound::persistence::IdentityRow;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{Email, IdentityId};
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
    async fn save(&self, identity: &Identity) -> Result<(), IdentityDomainError> {
        let row = IdentityRow::from_entity(identity);

        sqlx::query(
            "INSERT INTO identities \
            (id, email, email_verified_at, password_hash, created_at, updated_at) \
            VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(row.id)
        .bind(&row.email)
        .bind(&row.email_verified_at)
        .bind(&row.password_hash)
        .bind(row.created_at)
        .bind(row.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &IdentityId) -> Result<Option<Identity>, IdentityDomainError> {
        todo!()
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<Identity>, IdentityDomainError> {
        let email = email.as_str();
        let row = sqlx::query_as::<_, IdentityRow>(
            "SELECT id, email, email_verified_at, password_hash, created_at, updated_at \
            FROM identities \
            WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(IdentityRow::into_entity))
    }

    async fn find_all(&self) -> Result<Vec<Identity>, IdentityDomainError> {
        todo!()
    }

    async fn find_by_role(&self, role: &str) -> Result<Vec<Identity>, IdentityDomainError> {
        todo!()
    }

    async fn delete(&self, id: &IdentityId) -> Result<(), IdentityDomainError> {
        todo!()
    }
}
