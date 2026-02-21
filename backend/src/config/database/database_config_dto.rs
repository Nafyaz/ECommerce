use crate::config::database::DatabaseConfigError;
use crate::config::database::DatabaseConfigError::{InvalidConnectionLimits, InvalidTimeout};
use secrecy::SecretString;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfigDto {
    pub url: SecretString,

    pub max_connections: u32,

    pub min_connections: u32,

    #[serde(with = "humantime_serde")]
    pub connection_timeout: Duration,

    #[serde(with = "humantime_serde")]
    pub idle_timeout: Duration,

    #[serde(with = "humantime_serde")]
    pub statement_timeout: Duration,

    pub auto_migrate: bool,

    pub schema: String,

    pub log_statements: bool,

    #[serde(with = "humantime_serde")]
    pub log_slow_statements: Duration,
}

impl DatabaseConfigDto {
    fn validate(&self) -> Result<(), DatabaseConfigError> {
        if self.max_connections < self.min_connections {
            return Err(InvalidConnectionLimits {
                max_connections: self.max_connections,
                min_connections: self.min_connections,
            });
        }
        if self.connection_timeout.is_zero() {
            return Err(InvalidTimeout(self.connection_timeout));
        }
        Ok(())
    }
}
