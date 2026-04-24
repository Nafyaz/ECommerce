use crate::modules::identity::domain::value_objects::{Password, Phone};
use crate::modules::identity::errors::IdentityDomainError;
use secrecy::SecretString;

pub struct CreateUserByPhoneCommand {
    name: String,
    phone: Phone,
    password: Password,
}

impl CreateUserByPhoneCommand {
    pub fn new(name: impl Into<String>, phone: String, password: SecretString) -> Result<Self, IdentityDomainError> {
        let phone = Phone::new(phone)?;
        let password = Password::new(password)?;

        Ok(Self {
            name: name.into(),
            phone,
            password,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn phone(&self) -> &Phone {
        &self.phone
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
