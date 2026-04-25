use crate::modules::identity::application::command_results::RegisterResult;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl RegisterResponse {
    pub fn from_result(result: RegisterResult) -> Self {
        Self {
            id: result.id().to_owned(),
            created_at: result.created_at().to_owned(),
        }
    }
}
