use crate::modules::identity::domain::value_objects::{Email, OtpCode, OtpPurpose};
use crate::modules::identity::ports::outbound::NotificationPortError;
use async_trait::async_trait;

#[async_trait]
pub trait NotificationPort: Send + Sync {
    async fn send_otp_to_email(
        &self,
        email: &Email,
        otp_purpose: &OtpPurpose,
        otp_code: &OtpCode,
    ) -> Result<(), NotificationPortError>;
}
