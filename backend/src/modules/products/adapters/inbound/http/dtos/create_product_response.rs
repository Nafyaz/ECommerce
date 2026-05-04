use serde::Serialize;

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub id: String,
}
