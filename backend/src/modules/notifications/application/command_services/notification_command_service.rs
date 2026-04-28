use crate::modules::notifications::NotificationError;
use crate::modules::notifications::application::commands::SendEmailCommand;
use crate::modules::notifications::domain::entities::Notification;
use crate::modules::notifications::domain::value_objects::Channel;
use crate::modules::notifications::ports::inbound::NotificationCommandPort;
use crate::modules::notifications::ports::outbound::EmailProviderPort;
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
