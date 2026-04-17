use crate::modules::shared::AppError;
use crate::modules::users::application::commands::{CreateUserByEmailCommand, LoginByEmailCommand};
use crate::modules::users::domain::entities::User;
use crate::modules::users::domain::errors::UserDomainError;
use crate::modules::users::ports::inbound::UserCommandPort;
use crate::modules::users::ports::outbound::{PasswordHasherPort, TokenServicePort, UserRepositoryPort};
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
    async fn create_user_by_email(&self, command: CreateUserByEmailCommand) -> Result<User, AppError> {
        if let Some(_existing) = self.user_repo.find_by_email(&command.email).await? {
            return Err(UserDomainError::UserAlreadyExists.into());
        }

        let password_hash = self.password_hasher.hash_from_plain(&command.password)?;

        let user = User::new_by_email(command.name, command.email, password_hash);

        self.user_repo.save(&user).await?;

        // TODO: Publish event
        // TODO: Add tracing

        Ok(user)
    }

    async fn create_user_by_phone(&self, user: User) -> Result<User, AppError> {
        todo!()
    }

    async fn login_by_email(&self, command: LoginByEmailCommand) -> Result<User, AppError> {
        let user = self
            .user_repo
            .find_by_email(&command.email)
            .await?
            .ok_or(UserDomainError::InvalidCredentials)?;

        let is_valid = self.password_hasher.verify(&user.password_hash(), &command.password)?;

        if !is_valid {
            return Err(UserDomainError::InvalidCredentials.into());
        }

        let token = self.token_service.generate_token(&user.id())?;

        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<User, AppError> {
        todo!()
    }

    async fn delete_user(&self, user: User) -> Result<(), AppError> {
        todo!()
    }
}
