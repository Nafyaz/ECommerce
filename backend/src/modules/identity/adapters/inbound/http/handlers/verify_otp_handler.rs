use crate::modules::identity::IdentityState;
use crate::modules::identity::adapters::inbound::http::dtos::VerifyOtpRequest;
use crate::modules::identity::application::commands::VerifyOtpCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<StatusCode, AppError> {
    let command = VerifyOtpCommand::new(payload.identity_id, payload.otp_code)?;
    state.command_service.verify_otp(command).await?;

    Ok(StatusCode::NO_CONTENT)
}
