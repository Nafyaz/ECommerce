use crate::modules::identity::IdentityError;
use crate::modules::identity::application::command_results::{LoginResult, RegisterResult, VerifyOtpResult};
use crate::modules::identity::application::commands::{
    ForgotPasswordCommand, LoginCommand, RegisterCommand, ResendOtpCommand, VerifyOtpCommand,
};
use async_trait::async_trait;

#[async_trait]
pub trait IdentityCommandPort: Send + Sync {
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityError>;
    async fn resend_otp(&self, command: ResendOtpCommand) -> Result<(), IdentityError>;
    async fn verify_otp(&self, command: VerifyOtpCommand) -> Result<VerifyOtpResult, IdentityError>;
    async fn forgot_password(&self, command: ForgotPasswordCommand) -> Result<(), IdentityError>;
}
