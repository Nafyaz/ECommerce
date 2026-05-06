use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{OtpCode, OtpCodeHash};
use crate::modules::identity::ports::outbound::OtpProviderPort;
use hmac::{Hmac, Mac};
use rand::RngExt;
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

pub struct HmacOtpProvider {
    secret: SecretString,
}

impl HmacOtpProvider {
    pub fn new(secret: SecretString) -> Self {
        Self { secret }
    }
}

impl OtpProviderPort for HmacOtpProvider {
    fn generate_otp(&self) -> Result<OtpCode, IdentityError> {
        let mut rng = rand::rng();
        let code = SecretString::new(format!("{:06}", rng.random_range(0..=999999)).into());
        let otp_code = OtpCode::new(code)?;

        Ok(otp_code)
    }

    // TODO: What is happening in this code??!!!!
    fn hash_otp(&self, otp_code: &OtpCode) -> Result<OtpCodeHash, IdentityError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.expose_secret().as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(otp_code.expose().as_bytes());

        let result = mac.finalize().into_bytes();
        let code_hash = OtpCodeHash::from_str(hex::encode(result));

        Ok(code_hash)
    }

    fn verify_otp(&self, otp_code: &OtpCode, otp_code_hash: &OtpCodeHash) -> Result<bool, IdentityError> {
        let expected_bytes = hex::decode(otp_code_hash.as_str())
            .map_err(|_| IdentityError::InternalError("Failed to decode OTP hex".to_owned()))?;

        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.expose_secret().as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(otp_code.expose().as_bytes());

        Ok(mac.verify_slice(&expected_bytes).is_ok())
    }
}
