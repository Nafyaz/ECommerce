use crate::modules::product::domain::value_objects::{ContentType, FileSize, ObjectKey};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

pub struct PresignedUpload {
    pub upload_url: String,
    pub expires_at: DateTime<Utc>,
}

#[async_trait]
pub trait ObjectStoragePort: Send + Sync {
    async fn generate_presigned_url(
        &self,
        key: &ObjectKey,
        content_type: &ContentType,
        file_size: FileSize,
    ) -> Result<PresignedUpload, ObjectStorageError>;

    async fn check_object_exists(&self, key: &ObjectKey) -> Result<bool, ObjectStorageError>;

    async fn delete_object(&self, key: &ObjectKey) -> Result<(), ObjectStorageError>;
}

#[derive(Debug, Error)]
pub enum ObjectStorageError {
    #[error("storage unavailable")]
    Unavailable,

    #[error("object not found")]
    NotFound,

    #[error("invalid upload request")]
    InvalidRequest,

    #[error("unexpected storage error")]
    Unexpected,

    #[error("storage failure {0}")]
    StorageFailure(String),
}
