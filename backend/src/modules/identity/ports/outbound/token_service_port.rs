use serde::{Deserialize, Serialize};

use crate::modules::identity::domain::value_objects::UserId;
use crate::modules::shared::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub subject: String,
    pub expiration: usize,
    pub issued_at: usize,
}

pub trait TokenServicePort: Send + Sync {
    fn generate_token(&self, user_id: &UserId) -> Result<String, AppError>;
    fn validate_token(&self, token: &str) -> Result<Claims, AppError>;
}
