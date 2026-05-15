use crate::modules::identity::application::otp_app_error::OtpAppError;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpPurpose};
use uuid::Uuid;

pub struct ResendOtpCommand {
    identity_id: IdentityId,
    otp_purpose: OtpPurpose,
}

impl ResendOtpCommand {
    pub fn new(identity_id: Uuid, otp_purpose: String) -> Result<Self, OtpAppError> {
        let identity_id = IdentityId::from_uuid(identity_id);
        let otp_purpose = OtpPurpose::from_str(otp_purpose)?;

        Ok(Self {
            identity_id,
            otp_purpose,
        })
    }

    pub fn identity_id(&self) -> &IdentityId {
        &self.identity_id
    }

    pub fn otp_purpose(&self) -> &OtpPurpose {
        &self.otp_purpose
    }
}
