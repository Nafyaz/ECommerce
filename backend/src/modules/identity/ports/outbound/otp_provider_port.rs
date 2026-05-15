use crate::modules::identity::domain::value_objects::{OtpCode, OtpCodeHash};
use crate::modules::identity::ports::outbound::OtpProviderError;

pub trait OtpProviderPort: Send + Sync {
    fn generate_otp(&self) -> Result<OtpCode, OtpProviderError>;
    fn hash_otp(&self, otp_code: &OtpCode) -> Result<OtpCodeHash, OtpProviderError>;
    fn verify_otp(&self, otp_code: &OtpCode, otp_code_hash: &OtpCodeHash) -> Result<bool, OtpProviderError>;
}
