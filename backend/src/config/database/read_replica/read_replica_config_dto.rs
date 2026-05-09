use secrecy::SecretString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadReplicaConfigDto {
    pub enabled: bool,
    pub url: SecretString,
}
