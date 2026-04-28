use crate::modules::identity::IdentityError;
use crate::modules::identity::application::command_results::{LoginResult, RegisterResult};
use crate::modules::identity::application::commands::{LoginCommand, RegisterCommand};
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::ports::inbound::IdentityCommandPort;
use crate::modules::identity::ports::outbound::{IdentityRepositoryPort, PasswordHasherPort, TokenServicePort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct IdentityCommandService {
    identity_repo: Arc<dyn IdentityRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenServicePort>,
}

impl IdentityCommandService {
    pub fn new(
        identity_repo: Arc<dyn IdentityRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_service: Arc<dyn TokenServicePort>,
    ) -> Self {
        Self {
            identity_repo,
            password_hasher,
            token_service,
        }
    }
}

#[async_trait]
impl IdentityCommandPort for IdentityCommandService {
    async fn register(&self, command: RegisterCommand) -> Result<RegisterResult, IdentityError> {
        if let Some(_existing) = self.identity_repo.find_by_email(command.email()).await? {
            return Err(IdentityError::IdentityAlreadyExists.into());
        }

        let password_hash = self.password_hasher.hash_from_plain(command.password())?;

        let identity = Identity::new(command.email().to_owned(), password_hash)?;

        self.identity_repo.save(&identity).await?;

        // TODO: Publish event
        // TODO: Send email OTP

        tracing::info!(identity_id = %identity.id(), "Identity saved successfully");

        let result = RegisterResult::new(identity.id().as_uuid().to_owned(), identity.created_at());

        Ok(result)
    }

    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityError> {
        let user = self
            .identity_repo
            .find_by_email(command.email())
            .await?
            .ok_or(IdentityError::InvalidCredentials)?;

        let is_valid = self.password_hasher.verify(&user.password_hash(), command.password())?;

        if !is_valid {
            return Err(IdentityError::InvalidCredentials.into());
        }

        let token = self.token_service.generate_token(&user.id())?;

        tracing::info!(user_id = %user.id(), "User logged in successfully");

        let result = LoginResult { token };

        Ok(result)
    }
}
