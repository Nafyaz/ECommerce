use crate::modules::identity::domain::value_objects::{Password, Phone};
use crate::modules::identity::errors::UserDomainError;
use secrecy::SecretString;

pub struct CreateUserByPhoneCommand {
    pub name: String,
    pub phone: Phone,
    pub password: Password,
}

impl CreateUserByPhoneCommand {
    pub fn new(name: impl Into<String>, phone: String, password: SecretString) -> Result<Self, UserDomainError> {
        let phone = Phone::new(phone)?;
        let password = Password::new(password)?;

        Ok(Self {
            name: name.into(),
            phone,
            password,
        })
    }
}
