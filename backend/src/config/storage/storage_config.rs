use crate::config::storage::{StorageConfigDto, StorageConfigError};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

#[derive(Debug)]
pub struct StorageConfig {
    bucket_name: String,
    region: String,
    endpoint: String,
    access_key_id: SecretString,
    secret_access_key: SecretString,
    presigned_url_expiry: Duration,
}

impl StorageConfig {
    pub fn bucket_name(&self) -> &str {
        &self.bucket_name
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn access_key_id(&self) -> &SecretString {
        &self.access_key_id
    }

    pub fn secret_access_key(&self) -> &SecretString {
        &self.secret_access_key
    }

    pub fn presigned_url_expiry(&self) -> Duration {
        self.presigned_url_expiry
    }
}

impl TryFrom<StorageConfigDto> for StorageConfig {
    type Error = StorageConfigError;

    fn try_from(storage_config_dto: StorageConfigDto) -> Result<Self, Self::Error> {
        let bucket_name = validate_required_string("bucket_name", storage_config_dto.bucket_name)?;
        let region = validate_required_string("region", storage_config_dto.region)?;
        let endpoint = validate_endpoint(storage_config_dto.endpoint)?;

        if storage_config_dto.access_key_id.expose_secret().trim().is_empty() {
            return Err(StorageConfigError::EmptyAccessKeyId);
        }

        if storage_config_dto.secret_access_key.expose_secret().trim().is_empty() {
            return Err(StorageConfigError::EmptySecretAccessKey);
        }

        Ok(Self {
            bucket_name,
            region,
            endpoint,
            access_key_id: storage_config_dto.access_key_id,
            secret_access_key: storage_config_dto.secret_access_key,
            presigned_url_expiry: storage_config_dto.presigned_url_expiry,
        })
    }
}

fn validate_required_string(field: &'static str, value: String) -> Result<String, StorageConfigError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        return Err(StorageConfigError::EmptyField(field));
    }

    Ok(value)
}

fn validate_endpoint(endpoint: String) -> Result<String, StorageConfigError> {
    let endpoint = validate_required_string("endpoint", endpoint)?;

    if !(endpoint.starts_with("https://") || endpoint.starts_with("http://")) {
        return Err(StorageConfigError::InvalidEndpointScheme);
    }

    Ok(endpoint.trim_end_matches('/').to_owned())
}
