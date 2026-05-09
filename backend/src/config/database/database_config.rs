use super::{DatabaseConfigDto, DatabaseConfigError};
use crate::config::database::DatabaseConfigError::InvalidConnectionLimits;
use secrecy::SecretString;

#[derive(Debug)]
pub struct DatabaseConfig {
    url: SecretString,
    max_connections: u32,
    min_connections: u32,
    // connection_timeout: Duration,
    // statement_timeout: Duration,
    // read_replica: ReadReplicaConfig,
}

impl DatabaseConfig {
    pub fn url(&self) -> &SecretString {
        &self.url
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }
}

impl TryFrom<DatabaseConfigDto> for DatabaseConfig {
    type Error = DatabaseConfigError;

    fn try_from(database_config_dto: DatabaseConfigDto) -> Result<Self, Self::Error> {
        if database_config_dto.max_connections < database_config_dto.min_connections {
            return Err(InvalidConnectionLimits {
                max_connections: database_config_dto.max_connections,
                min_connections: database_config_dto.min_connections,
            });
        }

        Ok(Self {
            url: database_config_dto.url,
            max_connections: database_config_dto.max_connections,
            min_connections: database_config_dto.min_connections,
            // connection_timeout: database_config_dto.connection_timeout,
            // statement_timeout: database_config_dto.statement_timeout,
            // read_replica: database_config_dto.read_replica.try_into()?,
        })
    }
}
