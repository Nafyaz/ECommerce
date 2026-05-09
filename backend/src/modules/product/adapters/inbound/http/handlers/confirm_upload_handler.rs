use crate::modules::product::ProductHttpState;
use crate::modules::product::errors::ImageError;
use axum::extract::{Path, State};
use uuid::Uuid;

pub async fn handle(
    State(state): State<ProductHttpState>,
    Path((_product_id, image_id)): Path<(Uuid, Uuid)>,
) -> Result<ConfirmUploadResponse, ImageError> {
    state
        .product_image_command_service
        .confirm_product_image_upload(image_id)
        .await
}
