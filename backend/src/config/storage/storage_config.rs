use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorageConfig {
    pub provider: String,
    pub bucket: String,
    pub region: String,
    pub endpoint: Option<String>,
}
