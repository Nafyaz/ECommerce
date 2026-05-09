use crate::config::auth::AuthConfigError;
use crate::config::database::DatabaseConfigError;
use crate::config::server::ServerConfigError;
use crate::config::storage::StorageConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load configuration: {0}")]
    Load(#[from] config::ConfigError),

    #[error("invalid auth configuration: {0}")]
    Auth(#[from] AuthConfigError),

    #[error("invalid database configuration: {0}")]
    Database(#[from] DatabaseConfigError),

    #[error("invalid server configuration: {0}")]
    Server(#[from] ServerConfigError),

    #[error("invalid storage configuration: {0}")]
    Storage(#[from] StorageConfigError),
}
