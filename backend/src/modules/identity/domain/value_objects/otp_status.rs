use crate::modules::identity::domain::OtpDomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OtpStatus {
    Active,
    Expired,
    Consumed,
    Revoked,
}

impl OtpStatus {
    pub fn from_str(otp_status: impl Into<String>) -> Result<Self, OtpDomainError> {
        match otp_status.into().as_str() {
            "ACTIVE" => Ok(OtpStatus::Active),
            "EXPIRED" => Ok(OtpStatus::Expired),
            "CONSUMED" => Ok(OtpStatus::Consumed),
            "REVOKED" => Ok(OtpStatus::Revoked),
            os => Err(OtpDomainError::InvalidOtpStatus(os.into())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            OtpStatus::Active => "ACTIVE",
            OtpStatus::Expired => "EXPIRED",
            OtpStatus::Consumed => "CONSUMED",
            OtpStatus::Revoked => "REVOKED",
        }
    }
}
