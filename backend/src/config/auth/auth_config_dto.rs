use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthConfigDto {
    pub jwt_secret: SecretString,

    #[serde(with = "humantime_serde")]
    pub access_token_ttl: Duration,

    #[serde(with = "humantime_serde")]
    pub refresh_token_ttl: Duration,

    // TODO: should OTP be in auth?
    pub otp_secret: SecretString,
}
