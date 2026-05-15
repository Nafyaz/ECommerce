use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::product::ProductHttpState;
use crate::modules::product::adapters::inbound::http::ProductHttpError;
use crate::modules::product::adapters::inbound::http::dtos::{CreateUploadRequest, CreateUploadResponse};
use crate::modules::product::application::commands::CreateUploadCommand;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use uuid::Uuid;

pub async fn handle(
    State(state): State<ProductHttpState>,
    Path(product_id): Path<Uuid>,
    Extension(current_user): Extension<CurrentIdentity>,
    Json(payload): Json<CreateUploadRequest>,
) -> Result<(StatusCode, Json<CreateUploadResponse>), ProductHttpError> {
    let create_upload_command = CreateUploadCommand::new(
        current_user.identity_id,
        product_id,
        payload.content_type,
        payload.file_size,
        payload.display_order,
    )?;

    let result = state
        .product_image_command_service
        .create_upload(create_upload_command)
        .await?;

    let response = CreateUploadResponse::from(result);

    Ok((StatusCode::CREATED, Json(response)))
}
