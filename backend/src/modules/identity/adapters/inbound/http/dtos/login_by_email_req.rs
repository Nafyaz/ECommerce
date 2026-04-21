use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginByEmailRequest {
    pub email: String,
    pub password: SecretString,
}
