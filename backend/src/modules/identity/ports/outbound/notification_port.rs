use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, Otp};
use async_trait::async_trait;

#[async_trait]
pub trait NotificationPort: Send + Sync {
    async fn send_email_verification_otp(&self, email: &Email, otp: &Otp) -> Result<(), IdentityError>;
}
