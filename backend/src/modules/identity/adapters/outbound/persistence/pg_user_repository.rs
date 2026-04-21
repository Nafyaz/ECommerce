use crate::modules::identity::adapters::outbound::persistence::UserRow;
use crate::modules::identity::domain::entities::User;
use crate::modules::identity::domain::value_objects::{Email, Phone};
use crate::modules::identity::ports::outbound::UserRepositoryPort;
use crate::modules::shared::AppError;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryPort for PgUserRepository {
    async fn save(&self, user: &User) -> Result<(), AppError> {
        let row = UserRow::from_entity(user);

        sqlx::query(
            "INSERT INTO identity (id, name, email, email_verified_at, phone, phone_verified_at, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
            .bind(row.id).bind(&row.name).bind(&row.email).bind(&row.email_verified_at)
            .bind(&row.phone).bind(&row.phone_verified_at).bind(&row.password_hash)
            .bind(row.created_at).bind(row.updated_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        todo!()
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        let email = email.as_str();
        let row = sqlx::query_as::<_, UserRow> ("SELECT id, name, email, email_verified_at, phone, phone_verified_at, password_hash, created_at, updated_at FROM identity WHERE email = $1")
            .bind(email).fetch_optional(&self.pool).await?;

        Ok(row.map(UserRow::into_entity))
    }

    async fn find_by_phone(&self, phone: &Phone) -> Result<Option<User>, AppError> {
        todo!()
    }

    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        todo!()
    }

    async fn find_by_role(&self, role: &str) -> Result<Vec<User>, AppError> {
        todo!()
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        todo!()
    }
}
