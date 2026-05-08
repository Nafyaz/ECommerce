use crate::modules::user::domain::entities::User;
use crate::modules::user::domain::value_objects::{AccountId, UserId};
use crate::modules::user::errors::UserDomainError;
use async_trait::async_trait;

// TODO: All outbound ports should have their own errors

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), UserDomainError>;
    async fn find_by_account_id(&self, account_id: &AccountId) -> Result<Option<User>, UserDomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserDomainError>;
    async fn find_all(&self) -> Result<Vec<User>, UserDomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), UserDomainError>;
}
