use crate::modules::identity::IdentityHttpState;
use crate::modules::identity::application::commands::ForgotPasswordCommand;
use crate::modules::shared::AppError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityHttpState>,
    Json(payload): Json<ForgotPasswordCommand>,
) -> Result<StatusCode, AppError> {
    todo!()
}
