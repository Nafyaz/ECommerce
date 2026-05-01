use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpCodeHash, OtpId, OtpPurpose, OtpStatus};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct OtpRow {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub purpose: String,
    pub code_hash: String,
    pub status: String,
    pub attempts: u8,
    pub consumed_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl OtpRow {
    pub fn from_entity(otp: &Otp) -> Self {
        Self {
            id: otp.id().as_uuid().to_owned(),
            identity_id: otp.identity_id().as_uuid().to_owned(),
            purpose: otp.purpose().as_str().to_owned(),
            code_hash: otp.code_hash().as_str().to_owned(),
            status: otp.status().as_str().to_owned(),
            attempts: otp.attempts(),
            consumed_at: otp.consumed_at(),
            expires_at: otp.expires_at(),
            created_at: otp.created_at(),
        }
    }
}

impl TryFrom<OtpRow> for Otp {
    type Error = IdentityError;

    fn try_from(otp_row: OtpRow) -> Result<Self, Self::Error> {
        Otp::reconstitute(
            OtpId::from_uuid(otp_row.id),
            IdentityId::from_uuid(otp_row.identity_id),
            OtpPurpose::from_str(otp_row.purpose)?,
            OtpCodeHash::from_str(otp_row.code_hash),
            OtpStatus::from_str(otp_row.status)?,
            otp_row.attempts,
            otp_row.consumed_at,
            otp_row.expires_at,
            otp_row.created_at,
        )
    }
}
