use crate::modules::identity::IdentityState;
use crate::modules::identity::adapters::inbound::http::dtos::{VerifyOtpRequest, VerifyOtpResponse};
use crate::modules::identity::application::commands::VerifyOtpCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<(StatusCode, Json<VerifyOtpResponse>), AppError> {
    let command = VerifyOtpCommand::new(payload.email, payload.otp)?;
    let result = state.command_service.verify_otp(command).await?;
    let 

    Ok((StatusCode::OK, Json(VerifyOtpResponse { token: result.token })))
}
