use crate::modules::shared::AppError;
use crate::modules::users::adapters::inbound::http::dtos::{LoginByEmailRequest, LoginUserResponse};
use crate::modules::users::adapters::inbound::http::router::UserState;
use crate::modules::users::application::commands::LoginByEmailCommand;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn handle(
    State(state): State<UserState>,
    Json(payload): Json<LoginByEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    let command = LoginByEmailCommand::new(payload.email, payload.password)?;
    let result = state.command_service.login_by_email(command).await?;
    let response = LoginUserResponse { token: result.token };

    Ok((StatusCode::OK, Json(response)))
}
