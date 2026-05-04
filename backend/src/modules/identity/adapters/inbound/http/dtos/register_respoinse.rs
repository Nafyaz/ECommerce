use crate::modules::identity::application::command_results::RegisterResult;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: Uuid,
}

impl From<RegisterResult> for RegisterResponse {
    fn from(result: RegisterResult) -> Self {
        Self {
            id: result.id().to_owned(),
        }
    }
}
