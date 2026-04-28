use crate::modules::identity::IdentityError;
use crate::modules::identity::application::command_results::{LoginResult, RegisterResult};
use crate::modules::identity::application::commands::{LoginCommand, RegisterCommand};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityCommandPort: Send + Sync {
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityError>;
    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityError>;
}
