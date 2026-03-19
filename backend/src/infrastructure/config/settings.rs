use crate::infrastructure::config::SettingsError;
use crate::infrastructure::config::auth::AuthConfig;
use crate::infrastructure::config::database::DatabaseConfig;
use crate::infrastructure::config::server::ServerConfig;
use crate::infrastructure::config::settings_dto::SettingsDto;

#[derive(Debug)]
pub struct Settings {
    // auth: AuthConfig,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    // redis: RedisConfig,
    // storage: StorageConfig,
    // payments: PaymentConfig,
    // telemetry: ObservabilityConfig,
}

impl TryFrom<SettingsDto> for Settings {
    type Error = SettingsError;

    fn try_from(settings_dto: SettingsDto) -> Result<Self, Self::Error> {
        // validate settings_dto

        Ok(Self {
            // auth: settings.auth_dto.try_into()?,
            database: settings_dto.database.try_into()?,
            server: settings_dto.server.try_into()?,
        })
    }
}

impl Settings {
    pub fn load() -> Result<Self, SettingsError> {
        let settings_dto = SettingsDto::load()?;
        settings_dto.try_into()
    }
}
