use crate::modules::identity::IdentityDomainError;
use crate::modules::identity::application::command_results::{CreateUserResult, LoginResult};
use crate::modules::identity::application::commands::{CreateUserByEmailCommand, LoginByEmailCommand};
use crate::modules::identity::domain::entities::User;
use crate::modules::identity::ports::inbound::UserCommandPort;
use crate::modules::identity::ports::outbound::{PasswordHasherPort, TokenServicePort, UserRepositoryPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserCommandService {
    user_repo: Arc<dyn UserRepositoryPort>,
    password_hasher: Arc<dyn PasswordHasherPort>,
    token_service: Arc<dyn TokenServicePort>,
}

impl UserCommandService {
    pub fn new(
        user_repo: Arc<dyn UserRepositoryPort>,
        password_hasher: Arc<dyn PasswordHasherPort>,
        token_service: Arc<dyn TokenServicePort>,
    ) -> Self {
        Self {
            user_repo,
            password_hasher,
            token_service,
        }
    }
}

#[async_trait]
impl UserCommandPort for UserCommandService {
    async fn create_user_by_email(
        &self,
        command: CreateUserByEmailCommand,
    ) -> Result<CreateUserResult, IdentityDomainError> {
        if let Some(_existing) = self.user_repo.find_by_email(command.email()).await? {
            return Err(IdentityDomainError::UserAlreadyExists.into());
        }

        let password_hash = self.password_hasher.hash_from_plain(command.password())?;

        let user = User::new_by_email(command.name().to_owned(), command.email().to_owned(), password_hash)?;

        self.user_repo.save(&user).await?;

        // TODO: Publish event

        tracing::info!(user_id = %user.id(), "User registered successfully");

        let result = CreateUserResult::new(
            user.id().as_uuid().to_owned(),
            user.name().as_str().to_owned(),
            user.created_at(),
        );

        Ok(result)
    }

    async fn create_user_by_phone(&self, user: User) -> Result<User, IdentityDomainError> {
        todo!()
    }

    async fn login_by_email(&self, command: LoginByEmailCommand) -> Result<LoginResult, IdentityDomainError> {
        let user = self
            .user_repo
            .find_by_email(command.email())
            .await?
            .ok_or(IdentityDomainError::InvalidCredentials)?;

        let is_valid = self.password_hasher.verify(&user.password_hash(), command.password())?;

        if !is_valid {
            return Err(IdentityDomainError::InvalidCredentials.into());
        }

        let token = self.token_service.generate_token(&user.id())?;

        tracing::info!(user_id = %user.id(), "User logged in successfully");

        let result = LoginResult { token };

        Ok(result)
    }

    async fn update_user(&self, user: User) -> Result<User, IdentityDomainError> {
        todo!()
    }

    async fn delete_user(&self, user: User) -> Result<(), IdentityDomainError> {
        todo!()
    }
}
