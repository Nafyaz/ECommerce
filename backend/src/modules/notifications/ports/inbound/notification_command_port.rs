use crate::modules::notifications::NotificationError;
use crate::modules::notifications::application::commands::SendEmailCommand;
use async_trait::async_trait;

#[async_trait]
pub trait NotificationCommandPort: Send + Sync {
    async fn send_email(&self, command: SendEmailCommand) -> Result<(), NotificationError>;
}
