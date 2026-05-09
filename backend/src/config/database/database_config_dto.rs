use secrecy::SecretString;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfigDto {
    pub url: SecretString,
    pub max_connections: u32,
    pub min_connections: u32,
    //
    // #[serde(with = "humantime_serde")]
    // pub connection_timeout: Duration,
    //
    // #[serde(with = "humantime_serde")]
    // pub idle_timeout: Duration,
    //
    // #[serde(with = "humantime_serde")]
    // pub statement_timeout: Duration,
    //
    // pub auto_migrate: bool,
    //
    // pub schema: String,
    //
    // pub log_statements: bool,
    //
    // #[serde(with = "humantime_serde")]
    // pub log_slow_statements: Duration,
    //
    // pub read_replica: ReadReplicaConfigDto,
}
