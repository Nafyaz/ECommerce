use crate::modules::users::domain::entities::User;
use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::errors::UserDomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UserCommandPort: Send + Sync {
    async fn find_by_phone(&self, phone: &Phone) -> Result<Option<User>, UserDomainError>;
    async fn update_user(&self, user: User) -> Result<User, UserDomainError>;
    async fn delete_user(&self, user: User) -> Result<(), UserDomainError>;
}
