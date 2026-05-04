use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateVendorRequest {
    pub owner_id: Uuid,
    pub name: String,
}
