use crate::modules::identity::IdentityState;
use crate::modules::identity::adapters::inbound::http::dtos::ResendOtpRequest;
use crate::modules::identity::application::commands::ResendOtpCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityState>,
    Json(payload): Json<ResendOtpRequest>,
) -> Result<StatusCode, AppError> {
    let command = ResendOtpCommand::new(payload.identity_id, payload.otp_purpose)?;
    state.command_service.resend_otp(command).await?;

    Ok(StatusCode::NO_CONTENT)
}
