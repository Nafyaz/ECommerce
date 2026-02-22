use crate::config::database::read_replica::read_replica_config_dto::ReadReplicaConfigDto;
use crate::config::database::read_replica::read_replica_error::ReadReplicaError;
use secrecy::SecretString;

#[derive(Debug)]
pub struct ReadReplicaConfig {
    pub enabled: bool,
    pub url: SecretString,
}

impl TryFrom<ReadReplicaConfigDto> for ReadReplicaConfig {
    type Error = ReadReplicaError;

    fn try_from(read_replica_config_dto: ReadReplicaConfigDto) -> Result<Self, Self::Error> {
        read_replica_config_dto.validate()?;

        Ok(Self {
            enabled: read_replica_config_dto.enabled,
            url: read_replica_config_dto.url,
        })
    }
}
