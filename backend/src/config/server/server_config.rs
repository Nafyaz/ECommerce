use super::tls_config::TlsConfig;

#[derive(Debug)]
pub struct ServerConfig {
    host: String,
    port: u16,
    request_timeout: u64,
    shutdown_timeout: u64,
    tls: TlsConfig,
}
