use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservabilityConfig {
    pub log_level: String,
    pub enable_tracing: bool,
    pub otlp_endpoint: Option<String>,
    pub metrics_enabled: bool,
}
