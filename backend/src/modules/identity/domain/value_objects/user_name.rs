use crate::modules::identity::IdentityDomainError;
use std::fmt;

const NAME_MIN_LENGTH: usize = 3;
const NAME_MAX_LENGTH: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: impl Into<String>) -> Result<Self, IdentityDomainError> {
        let name = name.into();

        if name.is_empty() {
            return Err(IdentityDomainError::InvalidName("Name cannot be empty".into()));
        }

        if name.len() < NAME_MIN_LENGTH || name.len() > NAME_MAX_LENGTH {
            return Err(IdentityDomainError::InvalidName(format!(
                "Name must be between {} and {} characters long",
                NAME_MIN_LENGTH, NAME_MAX_LENGTH
            )));
        }

        Ok(Self(name))
    }

    pub fn from_str(value: String) -> Self {
        Self(value)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
