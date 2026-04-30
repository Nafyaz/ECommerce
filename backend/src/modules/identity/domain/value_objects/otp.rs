use crate::modules::identity::IdentityError;

pub struct Otp(String);

impl Otp {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();

        if value.len() != 6 || !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(IdentityError::InvalidOtp);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
