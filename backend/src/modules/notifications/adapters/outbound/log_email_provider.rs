use async_trait::async_trait;
use crate::modules::notifications::domain::entities::Notification;
use crate::modules::notifications::NotificationError;
use crate::modules::notifications::ports::outbound::EmailProviderPort;

pub struct LogEmailProvider;

#[async_trait]
impl EmailProviderPort for LogEmailProvider {
    async fn send_email(&self, notification: &Notification) -> Result<(), NotificationError> {
        let subject = notification.subject().map(|subject| subject.as_str()).unwrap_or("---No Subject---");

        tracing::info!(
            recipient = %notification.recipient(),
            subject = subject,
            body = notification.body().as_str(),
            "Sending email through log provider"
        );

        Ok(())
    }
}