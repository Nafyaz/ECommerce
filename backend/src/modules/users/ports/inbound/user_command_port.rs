use crate::modules::shared::AppError;
use crate::modules::users::application::commands::{CreateUserByEmailCommand, LoginByEmailCommand};
use crate::modules::users::domain::entities::User;
use async_trait::async_trait;

// TODO: use commands instead of entities
#[async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn create_user_by_email(&self, command: CreateUserByEmailCommand) -> Result<User, AppError>;
    async fn create_user_by_phone(&self, user: User) -> Result<User, AppError>;
    async fn login_by_email(&self, command: LoginByEmailCommand) -> Result<User, AppError>;
    async fn update_user(&self, user: User) -> Result<User, AppError>;
    async fn delete_user(&self, user: User) -> Result<(), AppError>;
}
