use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
