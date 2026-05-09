use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageConfigError {
    #[error("{0} must not be empty")]
    EmptyField(&'static str),

    #[error("endpoint must start with http:// or https://")]
    InvalidEndpointScheme,

    #[error("access_key_id must not be empty")]
    EmptyAccessKeyId,

    #[error("secret_access_key must not be empty")]
    EmptySecretAccessKey,
}
