use crate::modules::users::domain::errors::UserDomainError;
use crate::modules::users::domain::value_objects::{Email, Password, PasswordHash};
use secrecy::SecretString;

pub struct CreateUserByEmailCommand {
    pub name: String,
    pub email: Email,
    pub password: Password,
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
}
