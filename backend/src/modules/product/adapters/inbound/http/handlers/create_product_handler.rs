use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::product::adapters::inbound::http::ProductHttpState;
use crate::modules::product::adapters::inbound::http::dtos::{CreateProductRequest, CreateProductResponse};
use crate::modules::product::application::commands::CreateProductCommand;
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
        current_user.identity_id,
        payload.name,
        payload.supplier_id,
        payload.price_amount_minor,
        payload.price_currency,
    )?;
    let result = state.product_command_service.create_product(command).await?;
    let response = CreateProductResponse::from(result);

    Ok((StatusCode::CREATED, Json(response)))
}
