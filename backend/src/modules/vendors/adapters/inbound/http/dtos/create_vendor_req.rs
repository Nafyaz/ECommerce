use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateVendorRequest {
    pub name: String,
}
