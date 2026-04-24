use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::application::command_results::{CreateUserResult, LoginResult};
use crate::modules::identity::application::commands::{CreateUserByEmailCommand, LoginByEmailCommand};
use crate::modules::identity::domain::entities::User;
use async_trait::async_trait;

// TODO: move update / delete to User module
#[async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn create_user_by_email(
        &self,
        command: CreateUserByEmailCommand,
    ) -> Result<CreateUserResult, IdentityDomainError>;
    async fn create_user_by_phone(&self, user: User) -> Result<User, IdentityDomainError>;
    async fn login_by_email(&self, command: LoginByEmailCommand) -> Result<LoginResult, IdentityDomainError>;
    async fn update_user(&self, user: User) -> Result<User, IdentityDomainError>;
    async fn delete_user(&self, user: User) -> Result<(), IdentityDomainError>;
}
