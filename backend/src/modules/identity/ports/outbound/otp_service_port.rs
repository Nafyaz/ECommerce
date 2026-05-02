use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{OtpCode, OtpCodeHash};

pub trait OtpServicePort: Send + Sync {
    fn generate_otp(&self) -> Result<OtpCode, IdentityError>;
    fn hash_otp(&self, otp_code: &OtpCode) -> Result<OtpCodeHash, IdentityError>;
    fn verify_otp(&self, otp_code: &OtpCode, otp_code_hash: &OtpCodeHash) -> Result<bool, IdentityError>;
}
