use crate::modules::users::application::command_results::CreateUserResult;
use crate::modules::users::domain::entities::User;
use crate::modules::users::errors::UserDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn create_user(&self, user: &User) -> Result<CreateUserResult, UserDomainError>;
}
