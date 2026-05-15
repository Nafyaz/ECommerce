use crate::modules::identity::IdentityError;
use crate::modules::identity::application::IdentityAppError;
use crate::modules::identity::application::commands::{
    ForgotPasswordCommand, RegisterCommand, ResendOtpCommand, VerifyOtpCommand,
};
use crate::modules::identity::application::results::{RegisterResult, VerifyOtpResult};
use crate::modules::identity::domain::entities::{Identity, Otp};
use crate::modules::identity::domain::value_objects::OtpPurpose;
use crate::modules::identity::ports::inbound::IdentityCommandPort;
use crate::modules::identity::ports::outbound::{
    IdentityRepositoryPort, NotificationPort, OtpProviderPort, OtpRepositoryPort, PasswordHasherPort, TokenProviderPort,
};
use async_trait::async_trait;
use chrono::Duration;
use std::sync::Arc;

pub struct IdentityCommandService {
    identity_repo: Arc<dyn IdentityRepositoryPort>,
    notification_service: Arc<dyn NotificationPort>,
    otp_repo: Arc<dyn OtpRepositoryPort>,
    otp_service: Arc<dyn OtpProviderPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenProviderPort>,
}

impl IdentityCommandService {
    pub fn new(
        identity_repo: Arc<dyn IdentityRepositoryPort>,
        notification_service: Arc<dyn NotificationPort>,
        otp_service: Arc<dyn OtpProviderPort>,
        otp_repo: Arc<dyn OtpRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_service: Arc<dyn TokenProviderPort>,
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
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityAppError> {
        if let Some(_existing) = self.identity_repo.find_verified_by_email(command.email()).await? {
            return Err(IdentityAppError::VerifiedIdentityAlreadyExists);
        }

        let password_hash = self.password_hasher.hash_password(command.password())?;

        let identity = Identity::new(command.email().to_owned(), password_hash)?;

        self.identity_repo.save(&identity).await?;

        tracing::trace!(identity_id = %identity.id(), "Identity saved successfully");

        // TODO: Publish event
        // TODO: OTP generation / hashing / sending should be done by separate async worker
        // TODO: Take OTP Duration from auth_config
        let otp_code = self.otp_service.generate_otp()?;
        let otp_code_hash = self.otp_service.hash_otp(&otp_code)?;
        let otp = Otp::new(
            identity.id().to_owned(),
            OtpPurpose::Registration,
            otp_code_hash,
            Duration::minutes(10),
        )?;

        self.otp_repo.save(&otp).await?;
        tracing::trace!("OTP saved successfully");

        self.notification_service
            .send_otp_to_email(&identity.email(), &otp.purpose(), &otp_code)
            .await?;

        let result = RegisterResult::new(identity.id().as_uuid().to_owned());

        Ok(result)
    }

    // TODO: Add resend otp rate limit per user
    async fn resend_otp(&self, command: ResendOtpCommand) -> Result<(), IdentityAppError> {
        let identity = self
            .identity_repo
            .find_by_id(command.identity_id())
            .await?
            .ok_or(IdentityError::IdentityNotFound)?;
        tracing::trace!(identity_id = %identity.id(), "Identity found successfully");

        let mut old_otp = self
            .otp_repo
            .find_active(identity.id(), command.otp_purpose())
            .await?
            .ok_or(IdentityError::NoActiveOtp)?;

        old_otp.revoke();
        self.otp_repo.update(&old_otp).await?;
        tracing::trace!("Old OTP revoked successfully");

        let otp_code = self.otp_service.generate_otp()?;
        let otp_code_hash = self.otp_service.hash_otp(&otp_code)?;
        let otp = Otp::new(
            identity.id().to_owned(),
            command.otp_purpose().to_owned(),
            otp_code_hash,
            Duration::minutes(10),
        )?;

        self.otp_repo.save(&otp).await?;
        tracing::trace!("OTP saved successfully");

        self.notification_service
            .send_otp_to_email(&identity.email(), &otp.purpose(), &otp_code)
            .await?;

        Ok(())
    }

    async fn verify_otp(&self, command: VerifyOtpCommand) -> Result<VerifyOtpResult, IdentityAppError> {
        let mut identity = self
            .identity_repo
            .find_by_id(command.identity_id())
            .await?
            .ok_or(IdentityError::IdentityNotFound)?;
        tracing::trace!(identity_id = %identity.id(), "Identity found successfully");

        let mut otp = self
            .otp_repo
            .find_active(identity.id(), command.otp_purpose())
            .await?
            .ok_or(IdentityError::InvalidOtp)?;
        tracing::trace!("OTP found successfully");

        let is_valid = self.otp_service.verify_otp(&command.otp_code(), &otp.code_hash())?;
        if !is_valid {
            otp.increment_attempts();
            self.otp_repo.update(&otp).await?;
            tracing::trace!("OTP incremented successfully");
            return Err(IdentityError::InvalidOtp);
        }

        otp.consume();
        self.otp_repo.update(&otp).await?;
        tracing::trace!("OTP consumed successfully");

        if command.otp_purpose() == &OtpPurpose::Registration {
            identity.verify_identity();
            self.identity_repo.update(&identity).await?;
            tracing::trace!("Email verified successfully");

            //     TODO: Create user as here.
        }

        if let Some(token_type) = command.otp_purpose().issuing_token_after_verification() {
            let token = self.token_service.generate_token(identity.id(), token_type)?;
            return Ok(VerifyOtpResult::with_token(token));
        }

        Ok(VerifyOtpResult::without_token())
    }

    async fn forgot_password(&self, command: ForgotPasswordCommand) -> Result<(), IdentityAppError> {
        todo!()
    }
}
