use crate::modules::vendors::application::command_results::CreateVendorResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateVendorResponse {
    pub id: String,
}

impl From<CreateVendorResult> for CreateVendorResponse {
    fn from(result: CreateVendorResult) -> Self {
        Self {
            id: result.id.to_string(),
        }
    }
}
