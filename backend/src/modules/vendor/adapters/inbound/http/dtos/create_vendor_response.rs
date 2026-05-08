use crate::modules::vendor::application::command_results::CreateVendorResult;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateVendorResponse {
    pub id: Uuid,
}

impl From<CreateVendorResult> for CreateVendorResponse {
    fn from(result: CreateVendorResult) -> Self {
        Self { id: result.id }
    }
}
