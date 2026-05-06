use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub supplier_id: Uuid,
    pub price: u64,
    pub currency: String,
}
