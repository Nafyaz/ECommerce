use crate::modules::user::domain::value_objects::AccountId;
use crate::modules::user::domain::value_objects::phone::Phone;
use crate::modules::user::domain::value_objects::user_name::UserName;
use crate::modules::user::errors::UserDomainError;
use uuid::Uuid;

pub struct CreateUserCommand {
    account_id: AccountId,
    name: UserName,
    phone: Option<Phone>,
}

impl CreateUserCommand {
    pub fn new(account_id: Uuid, name: String, phone: Option<String>) -> Result<Self, UserDomainError> {
        let account_id = AccountId::from_uuid(account_id);
        let name = UserName::new(name)?;
        let phone = phone.map(|p| Phone::new(p)).transpose()?;

        Ok(CreateUserCommand {
            account_id,
            name,
            phone,
        })
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn phone(&self) -> &Option<Phone> {
        &self.phone
    }
}
