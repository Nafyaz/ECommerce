use super::tls_config_dto::TlsConfigDto;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct ServerConfigDto {
    pub host: String,
    port: u16,
    request_timeout: Duration,
    shutdown_timeout: Duration,
    tls: TlsConfigDto,
}
