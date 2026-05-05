use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub phone: Option<String>,
}
