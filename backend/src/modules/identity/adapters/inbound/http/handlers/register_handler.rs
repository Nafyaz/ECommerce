use crate::modules::identity::adapters::inbound::http::IdentityHttpState;
use crate::modules::identity::adapters::inbound::http::dtos::{RegisterRequest, RegisterResponse};
use crate::modules::identity::application::commands::RegisterCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityHttpState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), AppError> {
    let command = RegisterCommand::new(payload.email, payload.password)?;
    let result = state.command_service.register(command).await?;
    let response = RegisterResponse::from(result);

    Ok((StatusCode::CREATED, Json(response)))
}
