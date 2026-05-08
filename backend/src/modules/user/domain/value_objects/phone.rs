use crate::modules::user::errors::UserDomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Phone(String);

impl Phone {
    // TODO: Implement proper phone no validation
    pub fn new(phone: impl Into<String>) -> Result<Self, UserDomainError> {
        let phone = phone.into();
        if phone.is_empty() {
            return Err(UserDomainError::InvalidPhone("Phone cannot be empty".into()));
        }
        Ok(Self(phone))
    }

    pub fn from_str(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
