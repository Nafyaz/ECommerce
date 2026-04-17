use crate::infrastructure::config::auth::AuthConfig;
use crate::infrastructure::config::config_dto::ConfigDto;
use crate::infrastructure::config::config_error::ConfigError;
use crate::infrastructure::config::database::DatabaseConfig;
use crate::infrastructure::config::server::ServerConfig;

#[derive(Debug)]
pub struct Config {
    // auth: AuthConfig,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    // redis: RedisConfig,
    // storage: StorageConfig,
    // payments: PaymentConfig,
    // telemetry: ObservabilityConfig,
}

impl TryFrom<ConfigDto> for Config {
    type Error = ConfigError;

    fn try_from(settings_dto: ConfigDto) -> Result<Self, Self::Error> {
        // validate settings_dto

        Ok(Self {
            // auth: settings.auth_dto.try_into()?,
            database: settings_dto.database.try_into()?,
            server: settings_dto.server.try_into()?,
        })
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_dto = ConfigDto::load()?;
        config_dto.try_into()
    }
}
