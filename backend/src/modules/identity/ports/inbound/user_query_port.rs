use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::domain::entities::User;
use crate::modules::identity::domain::value_objects::UserId;
use async_trait::async_trait;

#[async_trait]
pub trait UserQueryPort: Send + Sync {
    // TODO: Use query objects instead of &str
    async fn get_user_by_id(&self, user_id: UserId) -> Result<User, IdentityDomainError>;
}
