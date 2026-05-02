use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::errors::UserDomainError;

pub struct CreateUserCommand {
    name: UserName,
    phone: Option<Phone>,
}

impl CreateUserCommand {
    pub fn new(name: String, phone: Option<String>) -> Result<Self, UserDomainError> {
        let name = UserName::new(name)?;
        let phone = phone.map(|p| Phone::new(p)).transpose()?;

        Ok(CreateUserCommand { name, phone })
    }
}
