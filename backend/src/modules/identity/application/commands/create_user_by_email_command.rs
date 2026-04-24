use crate::modules::identity::domain::value_objects::{Email, Password, UserName};
use crate::modules::identity::errors::IdentityDomainError;
use secrecy::SecretString;

pub struct CreateUserByEmailCommand {
    name: UserName,
    email: Email,
    password: Password,
}

impl CreateUserByEmailCommand {
    pub fn new(name: String, email: String, password: SecretString) -> Result<Self, IdentityDomainError> {
        let name = UserName::new(name)?;
        let email = Email::new(email)?;
        let password = Password::new(password)?;

        Ok(Self { name, email, password })
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}
