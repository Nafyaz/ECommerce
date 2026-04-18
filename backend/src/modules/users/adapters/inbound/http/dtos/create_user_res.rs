use crate::modules::users::application::command_results::CreateUserResult;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl CreateUserResponse {
    pub fn from_result(result: CreateUserResult) -> Self {
        Self {
            id: result.id.as_uuid().to_owned(),
            name: result.name,
            created_at: result.created_at,
        }
    }
}
