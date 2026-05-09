use crate::config::server::{ServerConfigDto, ServerConfigError};

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    // request_timeout: Duration,
    // shutdown_timeout: Duration,
    // tls: TlsConfig,
}

impl TryFrom<ServerConfigDto> for ServerConfig {
    type Error = ServerConfigError;

    fn try_from(server_config_dto: ServerConfigDto) -> Result<Self, Self::Error> {
        //TODO: validate server_config_dto

        Ok(Self {
            host: server_config_dto.host,
            port: server_config_dto.port,
            // request_timeout: server_config_dto.request_timeout,
            // shutdown_timeout: server_config_dto.shutdown_timeout,
            // tls: server_config_dto.tls.try_into()?,
        })
    }
}
