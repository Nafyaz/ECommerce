use crate::modules::identity::application::command_results::VerifyOtpResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyOtpResponse {
    pub token: String,
}

impl From<VerifyOtpResult> for VerifyOtpResponse {
    fn from(value: VerifyOtpResult) -> Self {
        Self {
            token: value.token().to_owned(),
        }
    }
}
