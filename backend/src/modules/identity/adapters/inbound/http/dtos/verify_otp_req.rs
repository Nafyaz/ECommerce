use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    pub email: String,
    pub otp: String,
}
