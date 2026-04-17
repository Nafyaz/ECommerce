use serde::{Deserialize, Serialize};

use crate::modules::shared::AppError;
use crate::modules::users::domain::value_objects::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub trait TokenServicePort: Send + Sync {
    fn generate_token(&self, user_id: &UserId, role: &str) -> Result<String, AppError>;
    fn validate_token(&self, token: &str) -> Result<Claims, AppError>;
}
