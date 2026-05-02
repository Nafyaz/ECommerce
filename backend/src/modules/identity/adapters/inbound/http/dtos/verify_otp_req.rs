use secrecy::SecretString;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub identity_id: Uuid,
    pub otp_purpose: String,
    pub otp_code: SecretString,
}
