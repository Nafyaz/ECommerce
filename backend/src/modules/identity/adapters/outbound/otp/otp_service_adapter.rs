use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{OtpCode, OtpCodeHash};
use crate::modules::identity::ports::outbound::OtpServicePort;
use rand::RngExt;
use secrecy::SecretString;

pub struct OtpServiceAdapter;

impl OtpServiceAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl OtpServicePort for OtpServiceAdapter {
    fn generate_otp(&self) -> Result<OtpCode, IdentityError> {
        let mut rng = rand::rng();
        let code = SecretString::new(format!("{:06}", rng.random_range(0..=999999)).into());
        let otp_code = OtpCode::new(code)?;

        Ok(otp_code)
    }

    fn hash_otp(&self, otp_code: &OtpCode) -> Result<OtpCodeHash, IdentityError> {
        todo!()
    }

    fn verify_otp(&self, otp_code: &OtpCode, otp_code_hash: &OtpCodeHash) -> Result<bool, IdentityError> {
        todo!()
    }
}
