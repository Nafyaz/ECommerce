use crate::modules::product::ProductHttpState;
use crate::modules::product::application::commands::ConfirmUploadCommand;
use crate::modules::shared::AppError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn handle(
    State(state): State<ProductHttpState>,
    Path((_product_id, image_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let confirm_upload_command = ConfirmUploadCommand::new(image_id)?;

    state
        .product_image_command_service
        .confirm_product_image_upload(confirm_upload_command)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
