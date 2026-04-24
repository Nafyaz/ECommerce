use crate::modules::identity::IdentityDomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, IdentityDomainError> {
        let email = email.into().trim().to_lowercase();

        if email.is_empty() {
            return Err(IdentityDomainError::InvalidEmail("Email cannot be empty".into()));
        }

        let parts = email.split_once('@').ok_or(IdentityDomainError::InvalidEmail(format!(
            "Invalid email format: {}",
            email
        )))?;

        if parts.0.is_empty() || parts.1.is_empty() {
            return Err(IdentityDomainError::InvalidEmail(format!(
                "Invalid email format: {}",
                email
            )));
        }

        let domain = parts.1;
        if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return Err(IdentityDomainError::InvalidEmail(format!(
                "Invalid email domain: {}",
                domain
            )));
        }

        Ok(Self(email))
    }

    pub fn from_str(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
