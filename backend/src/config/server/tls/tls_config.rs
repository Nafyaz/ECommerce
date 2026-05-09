use super::tls_config_dto::TlsConfigDto;
use super::tls_config_error::TlsConfigError;

#[derive(Debug)]
pub struct TlsConfig {
    enabled: bool,
    cert_path: String,
    key_path: String,
}

impl TryFrom<TlsConfigDto> for TlsConfig {
    type Error = TlsConfigError;

    fn try_from(tls_config_dto: TlsConfigDto) -> Result<Self, Self::Error> {
        // TODO: validate tls_config_dto

        Ok(Self {
            enabled: tls_config_dto.enabled,
            cert_path: tls_config_dto.cert_path,
            key_path: tls_config_dto.key_path,
        })
    }
}
