use secrecy::SecretString;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorageConfigDto {
    pub bucket_name: String,
    pub region: String,
    pub endpoint: String,
    pub access_key_id: SecretString,
    pub secret_access_key: SecretString,

    #[serde(with = "humantime_serde")]
    pub presigned_url_expiry: Duration,
}
