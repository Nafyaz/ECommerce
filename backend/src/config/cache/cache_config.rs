use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CacheConfig {
    pub url: String,
    pub pool_size: u32,
    pub ttl_default_seconds: u64,
}
