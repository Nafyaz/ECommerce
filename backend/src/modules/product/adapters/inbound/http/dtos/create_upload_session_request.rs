use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUploadSessionRequest {
    pub file_name: String,
    pub content_type: String,
    pub file_size: u64,
    pub display_order: u8,
}
