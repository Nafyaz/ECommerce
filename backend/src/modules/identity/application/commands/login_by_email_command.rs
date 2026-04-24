use crate::modules::identity::domain::value_objects::{Email, Password};
use crate::modules::identity::errors::IdentityDomainError;
use secrecy::SecretString;

pub struct LoginByEmailCommand {
    email: Email,
    password: Password,
}

impl LoginByEmailCommand {
    pub fn new(email: String, password: SecretString) -> Result<Self, IdentityDomainError> {
        let email = Email::new(email)?;
        let password = Password::new(password)?;

        Ok(Self { email, password })
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
