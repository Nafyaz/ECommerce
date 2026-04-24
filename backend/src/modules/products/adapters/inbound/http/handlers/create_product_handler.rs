use crate::modules::products::adapters::inbound::http::dtos::{CreateProductRequest, CreateProductResponse};
use crate::modules::products::adapters::inbound::http::router::ProductState;
use crate::modules::products::application::command_results::CreateProductResult;
use crate::modules::products::application::commands::CreateProductCommand;
use crate::modules::shared::AppError;
use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::infrastructure::http::dtos::CurrentUser;

pub async fn handle(
    State(state): State<ProductState>,
    Extension(current_user): Extension<CurrentUser>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<CreateProductResponse>), AppError> {
    let current_user =
    let command = CreateProductCommand::new(payload.name, payload.vendor_id, payload.price, payload.currency)?;
    let result = state.command_port.create_product(command).await;
    match result {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(CreateProductResponse {
                id: result.id.to_string(),
            }),
        )),
        Err(e) => Err(e.into()),
    }
}
