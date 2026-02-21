use crate::config::auth::AuthConfigError;
use crate::config::database::DatabaseConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("failed to load configuration")]
    Load(#[from] config::ConfigError),

    #[error("invalid auth configuration")]
    Auth(#[from] AuthConfigError),

    #[error("invalid database configuration")]
    Database(#[from] DatabaseConfigError),
}
