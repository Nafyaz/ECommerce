pub struct VerifyOtpResult {
    token: String,
}

impl VerifyOtpResult {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
