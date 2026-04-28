use crate::modules::notifications::NotificationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotificationBody(String);

impl NotificationBody {
    pub fn new(notification_body: impl Into<String>) -> Result<Self, NotificationError> {
        Ok(Self(notification_body.into()))
    }
}
