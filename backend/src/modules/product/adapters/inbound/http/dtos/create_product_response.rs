use crate::modules::product::application::results::CreateProductResult;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub id: Uuid,
}

impl From<CreateProductResult> for CreateProductResponse {
    fn from(result: CreateProductResult) -> Self {
        Self {
            id: result.id().to_owned(),
        }
    }
}
