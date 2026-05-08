use crate::modules::user::domain::value_objects::{AccountId, UserId};
use crate::modules::user::errors::UserDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserQueryPort: Send + Sync {
    async fn get_user_id_by_account(&self, account_id: &AccountId) -> Result<UserId, UserDomainError>;
}
