use crate::modules::notification::NotificationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotificationBody(String);

impl NotificationBody {
    pub fn new(notification_body: impl Into<String>) -> Result<Self, NotificationError> {
        Ok(Self(notification_body.into()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
