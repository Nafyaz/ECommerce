use crate::modules::identity::IdentityHttpState;
use crate::modules::identity::adapters::inbound::http::dtos::{VerifyOtpRequest, VerifyOtpResponse};
use crate::modules::identity::application::commands::VerifyOtpCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityHttpState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<(StatusCode, Json<VerifyOtpResponse>), AppError> {
    let command = VerifyOtpCommand::new(payload.identity_id, payload.otp_purpose, payload.otp_code)?;
    let result = state.command_service.verify_otp(command).await?;
    let response = VerifyOtpResponse::from(result);

    Ok((StatusCode::OK, Json(response)))
}
