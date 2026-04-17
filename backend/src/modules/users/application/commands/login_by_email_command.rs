use crate::modules::users::domain::errors::UserDomainError;
use crate::modules::users::domain::value_objects::{Email, Password};
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
