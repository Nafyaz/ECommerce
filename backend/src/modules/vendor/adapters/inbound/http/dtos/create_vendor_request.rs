use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateVendorRequest {
    pub name: String,
}
