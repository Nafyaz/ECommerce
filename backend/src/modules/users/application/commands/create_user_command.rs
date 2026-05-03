use crate::modules::users::domain::value_objects::IdentityId;
use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::errors::UserDomainError;
use uuid::Uuid;

pub struct CreateUserCommand {
    identity_id: IdentityId,
    name: UserName,
    phone: Option<Phone>,
}

impl CreateUserCommand {
    pub fn new(identity_id: Uuid, name: String, phone: Option<String>) -> Result<Self, UserDomainError> {
        let identity_id = IdentityId::from_uuid(identity_id);
        let name = UserName::new(name)?;
        let phone = phone.map(|p| Phone::new(p)).transpose()?;

        Ok(CreateUserCommand {
            identity_id,
            name,
            phone,
        })
    }

    pub fn identity_id(&self) -> &IdentityId {
        &self.identity_id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn phone(&self) -> &Option<Phone> {
        &self.phone
    }
}
