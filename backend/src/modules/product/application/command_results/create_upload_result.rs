use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUploadResult {
    id: Uuid,
    upload_url: String,
    expires_at: DateTime<Utc>,
}

impl CreateUploadResult {
    pub fn new(id: Uuid, upload_url: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id,
            upload_url,
            expires_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn upload_url(&self) -> String {
        self.upload_url.clone()
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}
