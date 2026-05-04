use crate::modules::identity::IdentityError;
use crate::modules::identity::application::command_results::LoginResult;
use crate::modules::identity::application::commands::LoginCommand;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::IdentityId;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityQueryPort: Send + Sync {
    async fn get_identity_by_id(&self, identity_id: &IdentityId) -> Result<Identity, IdentityError>;
    async fn is_verified(&self, identity_id: &IdentityId) -> Result<bool, IdentityError>;
    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityError>;
}
