use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ResendOtpRequest {
    pub identity_id: Uuid,
    pub otp_purpose: String,
}
