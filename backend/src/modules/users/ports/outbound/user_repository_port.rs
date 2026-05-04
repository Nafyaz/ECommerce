use crate::modules::users::domain::entities::User;
use crate::modules::users::domain::value_objects::{AuthIdentityId, UserId};
use crate::modules::users::errors::UserDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), UserDomainError>;
    async fn find_by_identity_id(&self, auth_identity_id: &AuthIdentityId) -> Result<Option<User>, UserDomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserDomainError>;
    async fn find_all(&self) -> Result<Vec<User>, UserDomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), UserDomainError>;
}
