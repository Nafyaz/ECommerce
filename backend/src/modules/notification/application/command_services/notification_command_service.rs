use crate::modules::notification::NotificationError;
use crate::modules::notification::application::commands::SendEmailCommand;
use crate::modules::notification::domain::entities::Notification;
use crate::modules::notification::domain::value_objects::Channel;
use crate::modules::notification::ports::inbound::NotificationCommandPort;
use crate::modules::notification::ports::outbound::EmailProviderPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct NotificationCommandService {
    email_provider: Arc<dyn EmailProviderPort>,
}

impl NotificationCommandService {
    pub fn new(email_provider: Arc<dyn EmailProviderPort>) -> Self {
        Self { email_provider }
    }
}

// TODO: Implement SendEmailResult
#[async_trait]
impl NotificationCommandPort for NotificationCommandService {
    async fn send_email(&self, command: SendEmailCommand) -> Result<(), NotificationError> {
        let notification = Notification::new(
            command.recipient().to_owned(),
            Channel::Email,
            Some(command.subject()),
            command.body().to_owned(),
        )?;

        self.email_provider.send_email(&notification).await?;

        tracing::info!(recipient = %command.recipient(), "Email sent successfully");

        Ok(())
    }
}
