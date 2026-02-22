#[derive(Debug)]
pub struct TlsConfig {
    enabled: bool,
    cert_path: String,
    key_path: String,
}
