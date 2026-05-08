use crate::modules::notification::NotificationError;
use crate::modules::notification::domain::entities::Notification;
use async_trait::async_trait;

#[async_trait]
pub trait EmailProviderPort: Send + Sync {
    async fn send_email(&self, notification: &Notification) -> Result<(), NotificationError>;
}
