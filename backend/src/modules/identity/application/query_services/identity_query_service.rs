use crate::modules::identity::application::command_results::LoginResult;
use crate::modules::identity::application::commands::LoginCommand;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{IdentityId, IdentityStatus, TokenType};
use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::identity::ports::outbound::{IdentityRepositoryPort, PasswordHasherPort};
use crate::modules::identity::{IdentityError, TokenProviderPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct IdentityQueryService {
    identity_repo: Arc<dyn IdentityRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenProviderPort>,
}

impl IdentityQueryService {
    pub fn new(
        identity_repo: Arc<dyn IdentityRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_service: Arc<dyn TokenProviderPort>,
    ) -> Self {
        Self {
            identity_repo,
            password_hasher,
            token_service,
        }
    }
}

#[async_trait]
impl IdentityQueryPort for IdentityQueryService {
    async fn get_identity_by_id(&self, identity_id: &IdentityId) -> Result<Identity, IdentityError> {
        todo!()
    }

    async fn check_verified(&self, identity_id: &IdentityId) -> Result<bool, IdentityError> {
        let identity = self
            .identity_repo
            .find_by_id(identity_id)
            .await?
            .ok_or(IdentityError::IdentityNotFound)?;

        Ok(*identity.status() == IdentityStatus::Verified)
    }

    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityError> {
        let identity = self
            .identity_repo
            .find_verified_by_email(command.email())
            .await?
            .ok_or(IdentityError::InvalidCredentials)?;

        let is_valid = self
            .password_hasher
            .verify_password(&identity.password_hash(), command.password())?;

        if !is_valid {
            return Err(IdentityError::InvalidCredentials.into());
        }

        let access_token = self.token_service.generate_token(identity.id(), &TokenType::Access)?;
        let refresh_token = self.token_service.generate_token(identity.id(), &TokenType::Refresh)?;

        tracing::info!(identity_id = %identity.id(), "User logged in successfully");

        let result = LoginResult::new(access_token, refresh_token);

        Ok(result)
    }
}
