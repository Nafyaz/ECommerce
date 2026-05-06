use crate::modules::products::domain::entities::Product;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ProductRecord {
    id: Uuid,
    name: String,
    description: Option<String>,
    supplier_id: Uuid,
    price_amount: i128,
    price_currency: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProductRecord {
    pub fn from_entity(product: &Product) -> Self {
        Self {
            id: product.id().as_uuid().to_owned(),
            name: product.name().as_str().to_owned(),
        }
    }
}
