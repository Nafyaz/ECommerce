use crate::modules::identity::application::results::LoginResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<LoginResult> for LoginResponse {
    fn from(login_result: LoginResult) -> Self {
        Self {
            access_token: login_result.access_token().to_owned(),
            refresh_token: login_result.refresh_token().to_owned(),
        }
    }
}
