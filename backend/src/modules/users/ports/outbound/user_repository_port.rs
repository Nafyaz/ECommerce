use crate::modules::shared::AppError;
use crate::modules::users::domain::entities::User;
use crate::modules::users::domain::value_objects::{Email, Phone};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), AppError>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError>;

    async fn find_by_phone(&self, phone: &Phone) -> Result<Option<User>, AppError>;

    async fn find_all(&self) -> Result<Vec<User>, AppError>;

    async fn find_by_role(&self, role: &str) -> Result<Vec<User>, AppError>;

    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
