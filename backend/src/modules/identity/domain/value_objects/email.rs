use crate::modules::identity::errors::UserDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, UserDomainError> {
        let email = email.into().trim().to_lowercase();

        if email.is_empty() {
            return Err(UserDomainError::InvalidEmail("Email cannot be empty".into()));
        }

        let parts: Vec<&str> = email.splitn(2, '@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(UserDomainError::InvalidEmail(format!(
                "Invalid email format: {}",
                email
            )));
        }

        let domain = parts[1];
        if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return Err(UserDomainError::InvalidEmail(format!(
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

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
