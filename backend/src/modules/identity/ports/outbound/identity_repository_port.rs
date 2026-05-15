use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{Email, IdentityId};
use crate::modules::identity::ports::outbound::IdentityRepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityRepositoryPort: Send + Sync {
    async fn save(&self, identity: &Identity) -> Result<(), IdentityRepositoryError>;
    async fn update(&self, identity: &Identity) -> Result<(), IdentityRepositoryError>;

    async fn find_by_id(&self, id: &IdentityId) -> Result<Option<Identity>, IdentityRepositoryError>;

    async fn find_verified_by_email(&self, email: &Email) -> Result<Option<Identity>, IdentityRepositoryError>;

    async fn find_all(&self) -> Result<Vec<Identity>, IdentityRepositoryError>;

    async fn find_by_role(&self, role: &str) -> Result<Vec<Identity>, IdentityRepositoryError>;

    async fn delete(&self, id: &IdentityId) -> Result<(), IdentityRepositoryError>;
}
