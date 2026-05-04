use crate::modules::users::application::command_results::CreateUserResult;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
}

impl From<CreateUserResult> for CreateUserResponse {
    fn from(create_user_result: CreateUserResult) -> Self {
        CreateUserResponse {
            id: create_user_result.id().to_owned(),
        }
    }
}
