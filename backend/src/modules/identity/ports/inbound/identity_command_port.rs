use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::application::command_results::{LoginResult, RegisterResult};
use crate::modules::identity::application::commands::{LoginCommand, RegisterCommand};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityCommandPort: Send + Sync {
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityDomainError>;
    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityDomainError>;
}
