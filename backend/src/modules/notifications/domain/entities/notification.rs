use crate::modules::notifications::NotificationError;
use crate::modules::notifications::domain::value_objects::{
    Channel, NotificationBody, NotificationId, NotificationStatus, Recipient, Subject,
};
use chrono::{DateTime, Utc};

// TODO: Build Notification log to store everything in DB. Use NoSql / redis. Is this a good idea in the first place?
// TODO: Build custom notification templates
pub struct Notification {
    id: NotificationId,
    recipient: Recipient,
    channel: Channel,
    subject: Option<Subject>,
    body: NotificationBody,
    status: NotificationStatus,
    created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        recipient: Recipient,
        channel: Channel,
        subject: Option<Subject>,
        body: NotificationBody,
    ) -> Result<Self, NotificationError> {
        let now = Utc::now();

        Ok(Self {
            id: NotificationId::new(),
            recipient,
            channel,
            subject,
            body,
            status: NotificationStatus::Pending,
            created_at: now,
        })
    }
}
