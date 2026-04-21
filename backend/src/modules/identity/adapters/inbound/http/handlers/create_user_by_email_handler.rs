use crate::modules::identity::adapters::inbound::http::dtos::{CreateUserByEmailRequest, CreateUserResponse};
use crate::modules::identity::adapters::inbound::http::router::UserState;
use crate::modules::identity::application::commands::CreateUserByEmailCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<UserState>,
    Json(payload): Json<CreateUserByEmailRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), AppError> {
    let command = CreateUserByEmailCommand::new(payload.name, payload.email, payload.password)?;
    let result = state.command_service.create_user_by_email(command).await?;
    let response = CreateUserResponse::from_result(result);

    Ok((StatusCode::CREATED, Json(response)))
}
