use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUploadRequest {
    pub content_type: String,
    pub file_size: i64,
    pub display_order: i32, //TODO: should I make it optional?
}
