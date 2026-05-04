use crate::modules::identity::application::command_results::VerifyOtpResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyOtpResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl From<VerifyOtpResult> for VerifyOtpResponse {
    fn from(value: VerifyOtpResult) -> Self {
        Self {
            token: value.token().map(str::to_owned),
        }
    }
}
