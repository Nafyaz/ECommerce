use crate::modules::notifications::NotificationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subject(String);

impl Subject {
    pub fn new(subject: impl Into<String>) -> Result<Self, NotificationError> {
        let subject = subject.into().trim().to_string();
        if subject.is_empty() {
            return Err(NotificationError::InvalidSubject(format!(
                "Subject cannot be empty: {}",
                subject
            )));
        }

        Ok(Self(subject))
    }
}
