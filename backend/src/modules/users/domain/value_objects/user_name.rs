use crate::modules::users::errors::UserDomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserName(String);

impl UserName {
    const NAME_MIN_LENGTH: usize = 3;
    const NAME_MAX_LENGTH: usize = 64;
    pub fn new(name: impl Into<String>) -> Result<Self, UserDomainError> {
        let name = name.into();

        if name.is_empty() {
            return Err(UserDomainError::InvalidName("Name cannot be empty".into()));
        }

        if name.len() < Self::NAME_MIN_LENGTH || name.len() > Self::NAME_MAX_LENGTH {
            return Err(UserDomainError::InvalidName(format!(
                "Name must be between {} and {} characters long",
                Self::NAME_MIN_LENGTH,
                Self::NAME_MAX_LENGTH
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
