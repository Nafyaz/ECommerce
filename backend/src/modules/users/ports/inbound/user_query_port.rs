use crate::modules::users::domain::value_objects::{IdentityId, UserId};
use crate::modules::users::errors::UserDomainError;

pub trait UserQueryPort {
    fn get_user_id_by_identity(&self, identity_id: IdentityId) -> Result<UserId, UserDomainError>;
}
