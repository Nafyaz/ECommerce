use crate::modules::identity::adapters::outbound::persistence::IdentityRecord;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{Email, IdentityId, IdentityStatus};
use crate::modules::identity::ports::outbound::{IdentityRepositoryError, IdentityRepositoryPort};
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

// TODO: Learn all about sqlx::Error
fn map_sqlx_error(err: sqlx::Error) -> IdentityRepositoryError {
    if let sqlx::Error::Database(database_error) = &err {
        return match database_error.code().as_deref() {
            Some("23505") | Some("23503") => IdentityRepositoryError::Conflict,
            _ => {
                tracing::error!("Identity repository database error: {:?}", err);
                IdentityRepositoryError::Unexpected
            }
        };
    }

    match err {
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => IdentityRepositoryError::Unavailable,
        sqlx::Error::RowNotFound => IdentityRepositoryError::NotFound,
        _ => {
            tracing::error!("Product repository database error: {:?}", err);
            IdentityRepositoryError::Unexpected
        }
    }
}

#[async_trait]
impl IdentityRepositoryPort for PgIdentityRepository {
    async fn save(&self, identity: &Identity) -> Result<(), IdentityRepositoryError> {
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
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn update(&self, identity: &Identity) -> Result<(), IdentityRepositoryError> {
        let record = IdentityRecord::from_entity(identity);

        let result = sqlx::query(
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
        .await
        .map_err(map_sqlx_error)?;

        if result.rows_affected() == 0 {
            return Err(IdentityRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn find_by_id(&self, id: &IdentityId) -> Result<Option<Identity>, IdentityRepositoryError> {
        let record = sqlx::query_as::<_, IdentityRecord>(
            "SELECT id, email, password_hash, status::TEXT, created_at, updated_at \
            FROM identities \
            WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(record.map(Identity::try_from).transpose()?)
    }

    async fn find_verified_by_email(&self, email: &Email) -> Result<Option<Identity>, IdentityRepositoryError> {
        let email = email.as_str();
        let record = sqlx::query_as::<_, IdentityRecord>(
            "SELECT id, email, password_hash, status::TEXT, created_at, updated_at \
            FROM identities \
            WHERE email = $1 AND status = $2::identity_status",
        )
        .bind(email)
        .bind(IdentityStatus::Verified.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(record.map(Identity::try_from).transpose()?)
    }

    async fn find_all(&self) -> Result<Vec<Identity>, IdentityRepositoryError> {
        todo!()
    }

    async fn find_by_role(&self, role: &str) -> Result<Vec<Identity>, IdentityRepositoryError> {
        todo!()
    }

    async fn delete(&self, id: &IdentityId) -> Result<(), IdentityRepositoryError> {
        todo!()
    }
}
