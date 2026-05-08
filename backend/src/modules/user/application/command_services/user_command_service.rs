use crate::modules::user::application::command_results::CreateUserResult;
use crate::modules::user::application::commands::CreateUserCommand;
use crate::modules::user::domain::entities::User;
use crate::modules::user::errors::UserDomainError;
use crate::modules::user::ports::inbound::UserCommandPort;
use crate::modules::user::ports::outbound::{UserIdentityPort, UserRepositoryPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserCommandService {
    user_identity_provider: Arc<dyn UserIdentityPort>,
    user_repo: Arc<dyn UserRepositoryPort>,
}

impl UserCommandService {
    pub fn new(user_identity_provider: Arc<dyn UserIdentityPort>, user_repo: Arc<dyn UserRepositoryPort>) -> Self {
        UserCommandService {
            user_identity_provider,
            user_repo,
        }
    }
}

#[async_trait]
impl UserCommandPort for UserCommandService {
    async fn create_user(&self, command: &CreateUserCommand) -> Result<CreateUserResult, UserDomainError> {
        let is_verified = self
            .user_identity_provider
            .check_verified(command.account_id())
            .await
            .map_err(|_| UserDomainError::IdentityPortError)?;

        if !is_verified {
            return Err(UserDomainError::AccountNotVerified(
                command.account_id().as_uuid().to_owned(),
            ));
        }

        if let Some(_existing) = self.user_repo.find_by_account_id(command.account_id()).await? {
            return Err(UserDomainError::UserAlreadyExists(_existing.id().as_uuid()));
        }

        let user = User::new(
            command.account_id().to_owned(),
            command.name().to_owned(),
            command.phone().to_owned(),
        )?;

        self.user_repo.save(&user).await?;

        Ok(CreateUserResult::new(user.id().as_uuid()))
    }
}
