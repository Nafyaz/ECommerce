use crate::modules::users::domain::value_objects::{AccountId, UserId};
use crate::modules::users::errors::UserDomainError;

pub trait UserQueryPort {
    fn get_user_id_by_account(&self, account_id: AccountId) -> Result<UserId, UserDomainError>;
}
