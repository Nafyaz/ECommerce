use crate::config::database::read_replica::read_replica_error::ReadReplicaError;
use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadReplicaConfigDto {
    pub enabled: bool,
    pub url: SecretString,
}

impl ReadReplicaConfigDto {
    pub fn validate(&self) -> Result<(), ReadReplicaError> {
        Ok(())
    }
}
