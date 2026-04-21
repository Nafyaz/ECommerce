use crate::modules::identity::domain::value_objects::{Email, Password};
use crate::modules::identity::errors::UserDomainError;
use secrecy::SecretString;

pub struct LoginByEmailCommand {
    pub email: Email,
    pub password: Password,
}

impl LoginByEmailCommand {
    pub fn new(email: String, password: SecretString) -> Result<Self, UserDomainError> {
        let email = Email::new(email)?;
        let password = Password::new(password)?;

        Ok(Self { email, password })
    }
}
