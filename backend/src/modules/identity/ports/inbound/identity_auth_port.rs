use crate::modules::identity::application::IdentityAppError;
use crate::modules::identity::application::commands::LoginCommand;
use crate::modules::identity::application::results::LoginResult;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityAuthPort: Send + Sync {
    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityAppError>;

    //     TODO: Add refresh token, logout
}
