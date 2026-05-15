use crate::modules::identity::application::IdentityAppError;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::IdentityId;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityQueryPort: Send + Sync {
    async fn get_identity_by_id(&self, identity_id: &IdentityId) -> Result<Identity, IdentityAppError>;
    async fn check_verified(&self, identity_id: &IdentityId) -> Result<bool, IdentityAppError>;
}
