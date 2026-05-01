use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{Email, OtpCode, OtpPurpose};
use crate::modules::identity::ports::outbound::NotificationPort;
use crate::modules::notifications::{NotificationState, SendEmailCommand};
use async_trait::async_trait;
use sha2::digest::typenum::op;

#[derive(Clone)]
pub struct NotificationModuleAdapter {
    notification_state: NotificationState,
}

impl NotificationModuleAdapter {
    pub fn new(notification_state: NotificationState) -> Self {
        Self { notification_state }
    }

    fn email_verification(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        let command = SendEmailCommand::new(
            email.as_str().to_owned(),
            "Verify your email".to_owned(),
            format!("Your verification code is {}.", otp_code.expose()),
        )?;

        Ok(command)
    }

    fn email_login(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        todo!()
    }
    fn email_reset_password(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
        todo!()
    }

    fn email_forgot_password(email: &Email, otp_code: &OtpCode) -> Result<SendEmailCommand, IdentityError> {
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
            OtpPurpose::EmailVerification => Self::email_verification(email, otp_code),
            OtpPurpose::PhoneVerification => Err(IdentityError::InternalError(
                "Phone verification not implemented".to_owned(),
            ))?,
            OtpPurpose::Login => Self::email_login(email, otp_code),
            OtpPurpose::PasswordReset => Self::email_forgot_password(email, otp_code),
            OtpPurpose::EmailChange => Self::email_change_email(email, otp_code),
            OtpPurpose::PasswordChange => Self::email_password_change(email, otp_code),
            OtpPurpose::DeleteAccount => Self::email_delete_account(email, otp_code),
        }?;

        self.notification_state.command_service.send_email(command).await?;

        Ok(())
    }
}
