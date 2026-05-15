use crate::modules::identity::adapters::inbound::http::{IdentityHttpError, IdentityHttpState};
use crate::modules::identity::application::commands::ForgotPasswordCommand;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn handle(
    State(state): State<IdentityHttpState>,
    Json(payload): Json<ForgotPasswordCommand>,
) -> Result<StatusCode, IdentityHttpError> {
    todo!()
}
