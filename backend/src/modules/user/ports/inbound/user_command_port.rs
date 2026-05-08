use crate::modules::user::application::command_results::CreateUserResult;
use crate::modules::user::application::commands::CreateUserCommand;
use crate::modules::user::errors::UserDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn create_user(&self, command: &CreateUserCommand) -> Result<CreateUserResult, UserDomainError>;
}
