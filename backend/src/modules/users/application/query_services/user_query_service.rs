use crate::modules::users::domain::value_objects::{AccountId, UserId};
use crate::modules::users::errors::UserDomainError;
use crate::modules::users::ports::inbound::UserQueryPort;
use crate::modules::users::ports::outbound::UserRepositoryPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserQueryService {
    user_repo: Arc<dyn UserRepositoryPort>,
}

impl UserQueryService {
    pub fn new(user_repo: Arc<dyn UserRepositoryPort>) -> Self {
        Self { user_repo }
    }
}

#[async_trait]
impl UserQueryPort for UserQueryService {
    async fn get_user_id_by_account(&self, account_id: &AccountId) -> Result<UserId, UserDomainError> {
        let user = self
            .user_repo
            .find_by_account_id(account_id)
            .await?
            .ok_or(UserDomainError::UserNotFound)?;

        Ok(user.id().to_owned())
    }
}
