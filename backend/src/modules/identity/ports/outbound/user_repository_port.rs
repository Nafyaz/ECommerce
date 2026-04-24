use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::domain::entities::User;
use crate::modules::identity::domain::value_objects::{Email, Phone, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), IdentityDomainError>;

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, IdentityDomainError>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, IdentityDomainError>;

    async fn find_by_phone(&self, phone: &Phone) -> Result<Option<User>, IdentityDomainError>;

    async fn find_all(&self) -> Result<Vec<User>, IdentityDomainError>;

    async fn find_by_role(&self, role: &str) -> Result<Vec<User>, IdentityDomainError>;

    async fn delete(&self, id: &UserId) -> Result<(), IdentityDomainError>;
}
