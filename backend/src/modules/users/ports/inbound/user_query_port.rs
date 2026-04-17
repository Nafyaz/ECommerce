use crate::modules::shared::AppError;
use crate::modules::users::domain::entities::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserQueryPort: Send + Sync {
    // TODO: Use query objects instead of &str
    async fn get_user_by_id(&self, user_id: &str) -> Result<User, AppError>;
}
