use chrono::{DateTime, Utc};

pub struct products {
    id: ProductId,
    name: String,
    description: Option<String>,
    vendor_id: VendorId,
    price_amount: i32,
    price_currency: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
