use crate::modules::identity::domain::value_objects::{Email, Password};
use crate::modules::identity::errors::IdentityError;
use secrecy::SecretString;

pub struct LoginCommand {
    email: Email,
    password: Password,
}

impl LoginCommand {
    pub fn new(email: String, password: SecretString) -> Result<Self, IdentityError> {
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
