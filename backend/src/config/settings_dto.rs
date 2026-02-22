use crate::config::app_env::AppEnv;
use crate::config::auth::AuthConfigDto;
use crate::config::database::DatabaseConfigDto;
use crate::config::server::ServerConfigDto;
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingsDto {
    pub server: ServerConfigDto,
    pub database: DatabaseConfigDto,

    // pub cache: CacheConfigDto,
    pub auth: AuthConfigDto,
    // pub session: SessionConfig,
    // pub payment: PaymentConfig,
    // pub storage: StorageConfig,
    // pub email: EmailConfig,
    // pub queue: QueueConfig,
    // pub search: SearchConfig,
    // pub rate_limit: RateLimitConfig,
    // pub cors: CorsConfig,
    // pub observability: ObservabilityConfig
}

impl SettingsDto {
    pub fn load() -> Result<Self, ConfigError> {
        let env = std::env::var("APP_ENV")
            .map_err(|e| ConfigError::Message(format!("APP_ENV must be set: {e}")))
            .and_then(|s| {
                s.parse::<AppEnv>()
                    .map_err(|e| ConfigError::Message(format!("Invalid APP_ENV: {e}")))
            })?;

        let config_dir = std::env::var("CONFIG_DIR")
            .map_err(|e| ConfigError::Message(format!("CONFIG_DIR must be set: {e}")))?;

        let config = Config::builder()
            .add_source(config::File::with_name(&format!("{config_dir}/base")).required(true))
            .add_source(config::File::with_name(&format!("{config_dir}/{env}")).required(false))
            .add_source(config::File::with_name(&format!("{config_dir}/local")).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true)
                    .list_separator(",")
                    .with_list_parse_key("cors.allowed_origins")
                    .with_list_parse_key("payment.supported_currencies")
                    .with_list_parse_key("cache.cluster.urls")
                    .with_list_parse_key("security.trusted_proxies"),
            )
            .build()?;

        let settings_dto = config.try_deserialize()?;

        Ok(settings_dto)
    }
}
