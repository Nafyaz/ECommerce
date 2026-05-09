use crate::config::auth::AuthConfig;
use crate::config::config_dto::ConfigDto;
use crate::config::config_error::ConfigError;
use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use crate::config::storage::StorageConfig;

#[derive(Debug)]
pub struct Config {
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub storage: StorageConfig,
    // redis: RedisConfig,
    // payments: PaymentConfig,
    // telemetry: ObservabilityConfig,
}

impl TryFrom<ConfigDto> for Config {
    type Error = ConfigError;

    fn try_from(configs_dto: ConfigDto) -> Result<Self, Self::Error> {
        // validate settings_dto

        Ok(Self {
            auth: configs_dto.auth.try_into()?,
            database: configs_dto.database.try_into()?,
            server: configs_dto.server.try_into()?,
            storage: configs_dto.storage.try_into()?,
        })
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_dto = ConfigDto::load()?;
        config_dto.try_into()
    }
}
