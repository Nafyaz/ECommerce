use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub supplier_id: Uuid,
    pub price_amount_minor: i64,
    pub price_currency: String,
}
