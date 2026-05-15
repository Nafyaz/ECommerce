pub struct VerifyOtpResult {
    token: Option<String>,
}

impl VerifyOtpResult {
    pub fn without_token() -> Self {
        Self { token: None }
    }

    pub fn with_token(token: String) -> Self {
        Self { token: Some(token) }
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
