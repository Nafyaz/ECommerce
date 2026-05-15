use crate::modules::identity::application::IdentityAppError;
use crate::modules::identity::application::commands::{
    ForgotPasswordCommand, RegisterCommand, ResendOtpCommand, VerifyOtpCommand,
};
use crate::modules::identity::application::results::{RegisterResult, VerifyOtpResult};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityCommandPort: Send + Sync {
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityAppError>;
    async fn resend_otp(&self, command: ResendOtpCommand) -> Result<(), IdentityAppError>;
    async fn verify_otp(&self, command: VerifyOtpCommand) -> Result<VerifyOtpResult, IdentityAppError>;
    async fn forgot_password(&self, command: ForgotPasswordCommand) -> Result<(), IdentityAppError>;
}
