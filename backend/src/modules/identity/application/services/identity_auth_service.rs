use crate::modules::identity::application::IdentityAppError;
use crate::modules::identity::application::commands::LoginCommand;
use crate::modules::identity::application::results::LoginResult;
use crate::modules::identity::domain::value_objects::TokenType;
use crate::modules::identity::ports::inbound::IdentityAuthPort;
use crate::modules::identity::ports::outbound::{IdentityRepositoryPort, PasswordHasherPort, TokenProviderPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct IdentityAuthService {
    identity_repo: Arc<dyn IdentityRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenProviderPort>,
}

impl IdentityAuthService {
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
impl IdentityAuthPort for IdentityAuthService {
    async fn login(&self, command: LoginCommand) -> Result<LoginResult, IdentityAppError> {
        let identity = self
            .identity_repo
            .find_verified_by_email(command.email())
            .await?
            .ok_or(IdentityAppError::InvalidCredentials)?;

        let is_valid = self
            .password_hasher
            .verify_password(&identity.password_hash(), command.password())?;

        if !is_valid {
            return Err(IdentityAppError::InvalidCredentials);
        }

        let access_token = self.token_service.generate_token(identity.id(), &TokenType::Access)?;
        let refresh_token = self.token_service.generate_token(identity.id(), &TokenType::Refresh)?;

        tracing::info!(identity_id = %identity.id(), "User logged in successfully");

        let result = LoginResult::new(access_token, refresh_token);

        Ok(result)
    }
}
