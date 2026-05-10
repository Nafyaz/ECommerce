use crate::modules::product::domain::value_objects::{ContentType, FileSize, ObjectKey};
use crate::modules::product::ports::outbound::{ObjectStorageError, ObjectStoragePort, PresignedUpload};
use async_trait::async_trait;
use aws_sdk_s3::presigning::PresigningConfig;
use chrono::Utc;
use std::time::Duration;

// TODO: Learn all about R2 and aws_sdk_s3
pub struct R2ObjectStorage {
    client: aws_sdk_s3::Client,
    bucket_name: String,
    presigned_url_expiry: Duration,
}

impl R2ObjectStorage {
    pub fn new(client: aws_sdk_s3::Client, bucket_name: String, presigned_url_expiry: Duration) -> Self {
        Self {
            client,
            bucket_name,
            presigned_url_expiry,
        }
    }
}

#[async_trait]
impl ObjectStoragePort for R2ObjectStorage {
    async fn generate_presigned_url(
        &self,
        key: &ObjectKey,
        content_type: &ContentType,
        file_size: FileSize,
    ) -> Result<PresignedUpload, ObjectStorageError> {
        let presigning_config = PresigningConfig::expires_in(self.presigned_url_expiry)
            .map_err(|e| ObjectStorageError::StorageFailure(e.to_string()))?;

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key.as_str())
            .content_type(content_type.as_str())
            .content_length(file_size.as_i64())
            .presigned(presigning_config)
            .await
            .map_err(|e| ObjectStorageError::StorageFailure(e.to_string()))?;

        Ok(PresignedUpload {
            upload_url: presigned_request.uri().to_string(),
            expires_at: Utc::now() + self.presigned_url_expiry,
        })
    }

    async fn check_object_exists(&self, key: &ObjectKey) -> Result<bool, ObjectStorageError> {
        todo!()
    }

    async fn delete_object(&self, key: &ObjectKey) -> Result<(), ObjectStorageError> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key.as_str())
            .send()
            .await
            .map_err(|e| ObjectStorageError::StorageFailure(e.to_string()))?;

        Ok(())
    }
}
