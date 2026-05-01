use crate::modules::identity::IdentityError;
use crate::modules::identity::application::command_results::{LoginResult, RegisterResult};
use crate::modules::identity::application::commands::{
    ForgotPasswordCommand, LoginCommand, RegisterCommand, ResendOtpCommand, VerifyOtpCommand,
};
use crate::modules::identity::domain::entities::{Identity, Otp};
use crate::modules::identity::domain::value_objects::OtpPurpose;
use crate::modules::identity::ports::inbound::IdentityCommandPort;
use crate::modules::identity::ports::outbound::{
    IdentityRepositoryPort, NotificationPort, OtpRepositoryPort, OtpServicePort, PasswordHasherPort, TokenServicePort,
};
use async_trait::async_trait;
use chrono::Duration;
use std::sync::Arc;

pub struct IdentityCommandService {
    identity_repo: Arc<dyn IdentityRepositoryPort>,
    notification_service: Arc<dyn NotificationPort>,
    otp_repo: Arc<dyn OtpRepositoryPort>,
    otp_service: Arc<dyn OtpServicePort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenServicePort>,
}

impl IdentityCommandService {
    pub fn new(
        identity_repo: Arc<dyn IdentityRepositoryPort>,
        notification_service: Arc<dyn NotificationPort>,
        otp_service: Arc<dyn OtpServicePort>,
        otp_repo: Arc<dyn OtpRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_service: Arc<dyn TokenServicePort>,
    ) -> Self {
        Self {
            identity_repo,
            notification_service,
            otp_service,
            otp_repo,
            password_hasher,
            token_service,
        }
    }
}

// TODO: run transactions
#[async_trait]
impl IdentityCommandPort for IdentityCommandService {
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityError> {
        if let Some(_existing) = self.identity_repo.find_verified_by_email(command.email()).await? {
            return Err(IdentityError::VerifiedIdentityAlreadyExists);
        }

        let password_hash = self.password_hasher.hash_from_plain(command.password())?;

        let identity = Identity::new(command.email().to_owned(), password_hash)?;

        self.identity_repo.save(&identity).await?;

        tracing::info!(identity_id = %identity.id(), "Identity saved successfully");

        // TODO: Publish event
        // TODO: OTP generation / hashing / sending should be done by separate async worker
        let otp_code = self.otp_service.generate_otp()?;
        let otp_code_hash = self.otp_service.hash_otp(&otp_code)?;
        let otp = Otp::new(
            identity.id().to_owned(),
            OtpPurpose::EmailVerification,
            otp_code_hash,
            Duration::minutes(10),
        )?;

        self.otp_repo.save(&otp).await?;

        self.notification_service
            .send_otp_to_email(&identity.email(), &otp.purpose(), &otp_code)
            .await?;

        let result = RegisterResult::new(identity.id().as_uuid().to_owned(), identity.created_at());

        Ok(result)
    }

    // TODO: Add resend otp rate limit per user
    async fn resend_otp(&self, command: ResendOtpCommand) -> Result<(), IdentityError> {
        let identity = self
            .identity_repo
            .find_by_id(command.identity_id())
            .await?
            .ok_or(IdentityError::IdentityNotFound)?;

        let otp_code = self.otp_service.generate_otp()?;
        let otp_code_hash = self.otp_service.hash_otp(&otp_code)?;
        let otp = Otp::new(
            identity.id().to_owned(),
            command.otp_purpose().to_owned(),
            otp_code_hash,
            Duration::minutes(10),
        )?;

        self.otp_repo.save(&otp).await?;

        self.notification_service
            .send_email_verification_otp(&identity.email(), &otp_code)
            .await?;

        Ok(())
    }

    async fn verify_otp(&self, command: VerifyOtpCommand) -> Result<(), IdentityError> {
        todo!()
    }

    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityError> {
        // TODO: There might be more than one identity with the same email. But only one should have verified email.
        // Use find_verified_by_email()
        let identity = self
            .identity_repo
            .find_by_email(command.email())
            .await?
            .ok_or(IdentityError::InvalidCredentials)?;

        let is_valid = self
            .password_hasher
            .verify(&identity.password_hash(), command.password())?;

        if !is_valid {
            return Err(IdentityError::InvalidCredentials.into());
        }

        let token = self.token_service.generate_token(&identity.id())?;

        tracing::info!(identity_id = %identity.id(), "User logged in successfully");

        let result = LoginResult { token };

        Ok(result)
    }

    async fn forgot_password(&self, command: ForgotPasswordCommand) -> Result<(), IdentityError> {
        todo!()
    }
}
