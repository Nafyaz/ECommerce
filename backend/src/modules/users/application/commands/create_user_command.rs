use crate::modules::users::domain::value_objects::AuthIdentityId;
use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::errors::UserDomainError;
use uuid::Uuid;

pub struct CreateUserCommand {
    auth_identity_id: AuthIdentityId,
    name: UserName,
    phone: Option<Phone>,
}

impl CreateUserCommand {
    pub fn new(auth_identity_id: Uuid, name: String, phone: Option<String>) -> Result<Self, UserDomainError> {
        let auth_identity_id = AuthIdentityId::from_uuid(auth_identity_id);
        let name = UserName::new(name)?;
        let phone = phone.map(|p| Phone::new(p)).transpose()?;

        Ok(CreateUserCommand {
            auth_identity_id,
            name,
            phone,
        })
    }

    pub fn auth_identity_id(&self) -> &AuthIdentityId {
        &self.auth_identity_id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn phone(&self) -> &Option<Phone> {
        &self.phone
    }
}
