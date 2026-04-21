use crate::modules::identity::domain::value_objects::{Email, Password};
use crate::modules::identity::errors::UserDomainError;
use secrecy::SecretString;

pub struct CreateUserByEmailCommand {
    name: String,
    email: Email,
    password: Password,
}

impl CreateUserByEmailCommand {
    pub fn new(name: impl Into<String>, email: String, password: SecretString) -> Result<Self, UserDomainError> {
        let email = Email::new(email)?;
        let password = Password::new(password)?;

        Ok(Self {
            name: name.into(),
            email,
            password,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
