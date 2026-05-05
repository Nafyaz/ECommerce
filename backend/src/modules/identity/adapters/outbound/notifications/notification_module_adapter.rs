use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, OtpCode, OtpPurpose};
use crate::modules::identity::ports::outbound::NotificationPort;
use crate::modules::notifications::{NotificationCommandPort, SendEmailCommand};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct NotificationModuleAdapter {
    notification_commands: Arc<dyn NotificationCommandPort>,
}

impl NotificationModuleAdapter {
    pub fn new(notification_commands: Arc<dyn NotificationCommandPort>) -> Self {
        Self { notification_commands }
    }

    fn email_registration(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        let command = SendEmailCommand::new(
            email.as_str().to_owned(),
            "Thank you for registering with us!".to_owned(),
            format!("Please verify your email address with code {}.", otp_code.expose()),
        )?;

        Ok(command)
    }

    fn email_verification(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        let command = SendEmailCommand::new(
            email.as_str().to_owned(),
            "Verify your email".to_owned(),
            format!("Your verification code is {}.", otp_code.expose()),
        )?;

        Ok(command)
    }

    fn email_reset_password(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        todo!()
    }

    fn email_change_email(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        todo!()
    }

    fn email_password_change(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        todo!()
    }

    fn email_delete_account(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        todo!()
    }
}

#[async_trait]
impl NotificationPort for NotificationModuleAdapter {
    async fn send_otp_to_email(
        &self,
        email: &Email,
        otp_purpose: &OtpPurpose,
        otp_code: &OtpCode,
    ) -> Result<(), IdentityError> {
        let command = match otp_purpose {
            OtpPurpose::Registration => Self::email_registration(email, otp_code),
            OtpPurpose::EmailVerification => Self::email_verification(email, otp_code),
            OtpPurpose::PhoneVerification => Err(IdentityError::InternalError(
                "Phone verification not implemented".to_owned(),
            ))?,
            OtpPurpose::PasswordReset => Self::email_reset_password(email, otp_code),
            OtpPurpose::EmailChange => Self::email_change_email(email, otp_code),
            OtpPurpose::PasswordChange => Self::email_password_change(email, otp_code),
            OtpPurpose::DeleteAccount => Self::email_delete_account(email, otp_code),
        }?;

        self.notification_commands.send_email(command).await?;

        Ok(())
    }
}
