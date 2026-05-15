use crate::modules::product::application::results::CreateUploadResult;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateUploadResponse {
    pub id: Uuid,
    pub presigned_url: String,
    pub expires_at: DateTime<Utc>,
}

impl From<CreateUploadResult> for CreateUploadResponse {
    fn from(create_upload_result: CreateUploadResult) -> Self {
        Self {
            id: create_upload_result.id(),
            presigned_url: create_upload_result.upload_url(),
            expires_at: create_upload_result.expires_at(),
        }
    }
}
