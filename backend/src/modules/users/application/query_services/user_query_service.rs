use crate::modules::shared::AppError;
use crate::modules::users::domain::entities::User;
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
    async fn get_user_by_id(&self, user_id: &str) -> Result<User, AppError> {
        todo!()
    }
}
