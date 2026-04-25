use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{Email, IdentityId};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityRepositoryPort: Send + Sync {
    async fn save(&self, identity: &Identity) -> Result<(), IdentityDomainError>;

    async fn find_by_id(&self, id: &IdentityId) -> Result<Option<Identity>, IdentityDomainError>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<Identity>, IdentityDomainError>;

    async fn find_all(&self) -> Result<Vec<Identity>, IdentityDomainError>;

    async fn find_by_role(&self, role: &str) -> Result<Vec<Identity>, IdentityDomainError>;

    async fn delete(&self, id: &IdentityId) -> Result<(), IdentityDomainError>;
}
