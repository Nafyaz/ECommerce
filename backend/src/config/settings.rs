use crate::config::SettingsError;
use crate::config::auth::AuthConfig;
use crate::config::database::DatabaseConfig;
use crate::config::server::ServerConfig;
use crate::config::settings_dto::SettingsDto;

#[derive(Debug)]
pub struct Settings {
    // auth: AuthConfig,
    database: DatabaseConfig,
    server: ServerConfig,
    // redis: RedisConfig,
    // storage: StorageConfig,
    // payment: PaymentConfig,
    // observability: ObservabilityConfig,
}

impl Settings {
    pub fn load() -> Result<Self, SettingsError> {
        let settings_dto = SettingsDto::load()?;
        settings_dto.try_into()
    }
}

impl TryFrom<SettingsDto> for Settings {
    type Error = SettingsError;

    fn try_from(settings_dto: SettingsDto) -> Result<Self, Self::Error> {
        // validate settings_dto

        Ok(Self {
            // auth: settings_dto.auth_dto.try_into()?,
            database: settings_dto.database_dto.try_into()?,
            server: settings_dto.server.try_into()?,
        })
    }
}
