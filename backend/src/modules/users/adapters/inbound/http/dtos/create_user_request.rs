use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub identity_id: Uuid,
    pub name: String,
    pub phone: Option<String>,
}
