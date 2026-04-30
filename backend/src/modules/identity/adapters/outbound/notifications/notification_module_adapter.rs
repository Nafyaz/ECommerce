use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, Otp};
use crate::modules::identity::ports::outbound::NotificationPort;
use crate::modules::notifications::{NotificationState, SendEmailCommand};
use async_trait::async_trait;

#[derive(Clone)]
pub struct NotificationModuleAdapter {
    notification_state: NotificationState,
}

impl NotificationModuleAdapter {
    pub fn new(notification_state: NotificationState) -> Self {
        Self { notification_state }
    }
}

#[async_trait]
impl NotificationPort for NotificationModuleAdapter {
    async fn send_email_verification_otp(&self, email: &Email, otp: &Otp) -> Result<(), IdentityError> {
        let command = SendEmailCommand::new(
            email.as_str().to_owned(),
            "Verify your email".to_owned(),
            format!("Your verification code is {}.", otp.as_str()),
        )?;

        self.notification_state.command_service.send_email(command).await?;

        Ok(())
    }
}
