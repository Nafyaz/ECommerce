use crate::modules::identity::adapters::inbound::http::dtos::{LoginRequest, LoginResponse};
use crate::modules::identity::adapters::inbound::http::router::IdentityState;
use crate::modules::identity::application::commands::LoginCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), AppError> {
    let command = LoginCommand::new(payload.email, payload.password)?;
    let result = state.command_service.login(command).await?;
    let response = LoginResponse { token: result.token };

    Ok((StatusCode::OK, Json(response)))
}
