use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::shared::AppError;
use crate::modules::users::adapters::inbound::http::dtos::{CreateUserRequest, CreateUserResponse};
use crate::modules::users::adapters::inbound::http::user_http_state::UserHttpState;
use crate::modules::users::application::commands::CreateUserCommand;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

pub async fn handle(
    State(state): State<UserHttpState>,
    Extension(current_identity): Extension<CurrentIdentity>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), AppError> {
    let command = CreateUserCommand::new(current_identity.identity_id, payload.name, payload.phone)?;
    let result = state.command_service.create_user(&command).await?;
    let response = CreateUserResponse::from(result);

    Ok((StatusCode::CREATED, Json(response)))
}
