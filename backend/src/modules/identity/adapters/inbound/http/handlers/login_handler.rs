use crate::modules::identity::adapters::inbound::http::dtos::{LoginRequest, LoginResponse};
use crate::modules::identity::adapters::inbound::http::{IdentityHttpError, IdentityHttpState};
use crate::modules::identity::application::commands::LoginCommand;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityHttpState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), IdentityHttpError> {
    let command = LoginCommand::new(payload.email, payload.password)?;
    let result = state.auth_service.login(command).await?;
    let response = LoginResponse::from(result);

    Ok((StatusCode::OK, Json(response)))
}
