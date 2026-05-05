use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::shared::AppError;
use crate::modules::vendors::adapters::inbound::http::VendorHttpState;
use crate::modules::vendors::adapters::inbound::http::dtos::{CreateVendorRequest, CreateVendorResponse};
use crate::modules::vendors::application::commands::CreateVendorCommand;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

pub async fn handle(
    State(state): State<VendorHttpState>,
    Extension(current_identity): Extension<CurrentIdentity>,
    Json(payload): Json<CreateVendorRequest>,
) -> Result<(StatusCode, Json<CreateVendorResponse>), AppError> {
    let command = CreateVendorCommand::new(current_identity.identity_id, payload.name)?;
    let result = state.command_service.create_vendor(command).await?;
    let response = CreateVendorResponse::from(result);

    Ok((StatusCode::CREATED, Json(response)))
}
