use crate::modules::shared::AppError;
use crate::modules::users::adapters::inbound::http::dtos::{CreateUserByEmailRequest, CreateUserResponse};
use crate::modules::users::adapters::inbound::http::router::UserState;
use crate::modules::users::application::commands::CreateUserByEmailCommand;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn handle(
    State(state): State<UserState>,
    Json(payload): Json<CreateUserByEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = CreateUserByEmailCommand::new(payload.name, payload.email, payload.password)?;
    let user = state.command_service.create_user_by_email(command).await?;
    let response = CreateUserResponse::from_entity(&user);

    Ok((StatusCode::CREATED, Json(response)))
}
