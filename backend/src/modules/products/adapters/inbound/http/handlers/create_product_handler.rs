use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::products::adapters::inbound::http::ProductHttpState;
use crate::modules::products::adapters::inbound::http::dtos::{CreateProductRequest, CreateProductResponse};
use crate::modules::products::application::commands::CreateProductCommand;
use crate::modules::shared::AppError;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

pub async fn handle(
    State(state): State<ProductHttpState>,
    Extension(current_user): Extension<CurrentIdentity>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<CreateProductResponse>), AppError> {
    let command = CreateProductCommand::new(
        payload.name,
        payload.vendor_id,
        payload.price,
        payload.currency,
        current_user.id,
    )?;
    let result = state.command_port.create_product(command).await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateProductResponse {
            id: result.product_id().to_string(),
        }),
    ))
}
