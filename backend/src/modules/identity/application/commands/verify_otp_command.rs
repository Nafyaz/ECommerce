use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, IdentityId, OtpCode};
use secrecy::SecretString;
use uuid::Uuid;

pub struct VerifyOtpCommand {
    identity_id: IdentityId,
    otp_code: OtpCode,
}

impl VerifyOtpCommand {
    pub fn new(identity_id: Uuid, otp_code: SecretString) -> Result<Self, IdentityError> {
        let identity_id = IdentityId::from_uuid(identity_id);
        let otp_code = OtpCode::new(otp_code)?;

        Ok(Self { identity_id, otp_code })
    }

    pub fn identity_id(&self) -> &IdentityId {
        &self.identity_id
    }

    pub fn otp_code(&self) -> &OtpCode {
        &self.otp_code
    }
}
