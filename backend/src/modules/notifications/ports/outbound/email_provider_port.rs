use crate::modules::notifications::NotificationError;
use crate::modules::notifications::domain::entities::Notification;
use async_trait::async_trait;

#[async_trait]
pub trait EmailProviderPort: Send + Sync {
    async fn send_email(&self, notification: &Notification) -> Result<(), NotificationError>;
}
