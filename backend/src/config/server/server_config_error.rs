use super::tls_config_error::TlsConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerConfigError {
    #[error("TLS config failed: {0}")]
    TlsError(#[from] TlsConfigError),
}
