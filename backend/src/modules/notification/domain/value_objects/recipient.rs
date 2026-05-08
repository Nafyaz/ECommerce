use crate::modules::notification::NotificationError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Recipient {
    Email(String),
    Phone(String),
}

impl Recipient {
    pub fn email(value: impl Into<String>) -> Result<Self, NotificationError> {
        let email = value.into().trim().to_lowercase();

        if email.is_empty() {
            return Err(NotificationError::InvalidRecipient(
                "Recipient email cannot be empty".into(),
            ));
        }

        let parts = email
            .split_once('@')
            .ok_or(NotificationError::InvalidRecipient(format!(
                "Invalid email format: {}",
                email
            )))?;

        if parts.0.is_empty() || parts.1.is_empty() {
            return Err(NotificationError::InvalidRecipient(format!(
                "Invalid email format: {}",
                email
            )));
        }

        let domain = parts.1;
        if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return Err(NotificationError::InvalidRecipient(format!(
                "Invalid email domain: {}",
                domain
            )));
        }

        Ok(Self::Email(email))
    }

    pub fn phone(value: impl Into<String>) -> Result<Self, NotificationError> {
        let phone = value.into().trim().to_string();

        if phone.is_empty() {
            return Err(NotificationError::InvalidRecipient(
                "Recipient phone cannot be empty".into(),
            ));
        }

        Ok(Self::Phone(phone))
    }

    pub fn as_email(&self) -> Option<&str> {
        match self {
            Self::Email(value) => Some(value),
            Self::Phone(_) => None,
        }
    }

    pub fn as_phone(&self) -> Option<&str> {
        match self {
            Self::Phone(value) => Some(value),
            Self::Email(_) => None,
        }
    }
}

impl fmt::Display for Recipient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email(value) | Self::Phone(value) => write!(f, "{}", value),
        }
    }
}
