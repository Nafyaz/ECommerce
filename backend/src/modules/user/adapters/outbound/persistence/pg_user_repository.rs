use crate::modules::user::adapters::outbound::persistence::user_record::UserRecord;
use crate::modules::user::domain::entities::User;
use crate::modules::user::domain::value_objects::{AccountId, UserId};
use crate::modules::user::errors::UserDomainError;
use crate::modules::user::ports::outbound::UserRepositoryPort;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for UserDomainError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);
        UserDomainError::InternalError(format!("Database error: {}", err))
    }
}

#[async_trait]
impl UserRepositoryPort for PgUserRepository {
    async fn save(&self, user: &User) -> Result<(), UserDomainError> {
        let row = UserRecord::from_entity(user);

        sqlx::query(
            "INSERT INTO users \
        (id, account_id, name, phone, phone_verified_at, created_at, updated_at) \
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id())
        .bind(row.account_id())
        .bind(row.name())
        .bind(row.phone())
        .bind(row.phone_verified_at())
        .bind(row.created_at())
        .bind(row.updated_at())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_account_id(&self, identity_id: &AccountId) -> Result<Option<User>, UserDomainError> {
        let row = sqlx::query_as::<_, UserRecord>(
            "SELECT id, account_id, name, phone, phone_verified_at, updated_at, created_at
            FROM users
            WHERE account_id = $1",
        )
        .bind(identity_id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(User::try_from).transpose()?)
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserDomainError> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<User>, UserDomainError> {
        todo!()
    }

    async fn delete(&self, id: &UserId) -> Result<(), UserDomainError> {
        todo!()
    }
}
