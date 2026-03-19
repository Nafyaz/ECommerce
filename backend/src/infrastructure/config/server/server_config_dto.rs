use super::tls_config_dto::TlsConfigDto;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct ServerConfigDto {
    pub host: String,
    pub port: u16,
    // #[serde(with = "humantime_serde")]
    // pub request_timeout: Duration,
    //
    // #[serde(with = "humantime_serde")]
    // pub shutdown_timeout: Duration,
    // pub tls: TlsConfigDto,
}
