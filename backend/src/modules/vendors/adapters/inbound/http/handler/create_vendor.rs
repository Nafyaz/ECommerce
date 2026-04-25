use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::shared::AppError;
use crate::modules::vendors::adapters::inbound::http::dtos::{CreateVendorRequest, CreateVendorResponse};
use crate::modules::vendors::adapters::inbound::http::router::VendorState;
use crate::modules::vendors::application::commands::CreateVendorCommand;
use crate::modules::vendors::domain::value_objects::OwnerId;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

pub async fn handle(
    State(state): State<VendorState>,
    Extension(current_identity): Extension<CurrentIdentity>,
    Json(payload): Json<CreateVendorRequest>,
) -> Result<(StatusCode, Json<CreateVendorResponse>), AppError> {
    // TODO: owner_id should come from user module
    let owner_id = OwnerId::from_uuid(current_identity.id);
    let command = CreateVendorCommand::new(payload.name)?;
    let result = state.command_service.create_vendor(command, owner_id).await?;
    let response = CreateVendorResponse::from_result(result);

    Ok((StatusCode::CREATED, Json(response)))
}
