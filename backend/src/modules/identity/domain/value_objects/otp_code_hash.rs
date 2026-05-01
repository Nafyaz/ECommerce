#[derive(Debug, Clone)]
pub struct OtpCodeHash(String);

impl OtpCodeHash {
    pub fn from_str(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
