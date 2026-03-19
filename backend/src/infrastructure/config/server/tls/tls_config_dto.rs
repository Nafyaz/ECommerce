use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TlsConfigDto {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
}
