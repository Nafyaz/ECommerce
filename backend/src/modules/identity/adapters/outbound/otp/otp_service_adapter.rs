use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{OtpCode, OtpCodeHash};
use crate::modules::identity::ports::outbound::OtpServicePort;
use hmac::{Hmac, Mac};
use rand::RngExt;
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

pub struct OtpServiceAdapter {
    secret: SecretString,
}

impl OtpServiceAdapter {
    pub fn new(secret: SecretString) -> Self {
        Self { secret }
    }
}

impl OtpServicePort for OtpServiceAdapter {
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

        let result = mac.finalize();
        let bytes = result.into_bytes();
        let hex_string: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        let code_hash = OtpCodeHash::from_str(hex_string);

        Ok(code_hash)
    }

    fn verify_otp(&self, otp_code: &OtpCode, otp_code_hash: &OtpCodeHash) -> bool {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.expose_secret().as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(otp_code.expose().as_bytes());

        mac.verify_slice(otp_code_hash.as_str().as_bytes()).is_ok()
    }
}
