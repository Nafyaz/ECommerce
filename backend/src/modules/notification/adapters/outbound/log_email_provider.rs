use crate::modules::notification::NotificationError;
use crate::modules::notification::domain::entities::Notification;
use crate::modules::notification::ports::outbound::EmailProviderPort;
use async_trait::async_trait;

pub struct LogEmailProvider;

#[async_trait]
impl EmailProviderPort for LogEmailProvider {
    async fn send_email(&self, notification: &Notification) -> Result<(), NotificationError> {
        let subject = notification
            .subject()
            .map(|subject| subject.as_str())
            .unwrap_or("---No Subject---");

        tracing::info!(
            recipient = %notification.recipient(),
            subject = subject,
            body = notification.body().as_str(),
            "Sending email through log provider"
        );

        Ok(())
    }
}
