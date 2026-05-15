use crate::modules::identity::domain::IdentityDomainError;

// TODO: Implement functionalities for expired and locked
#[derive(PartialEq)]
pub enum IdentityStatus {
    Pending,
    Verified,
    Expired,
    Locked,
}

impl IdentityStatus {
    pub fn from_str(identity_status: impl Into<String>) -> Result<IdentityStatus, IdentityDomainError> {
        match identity_status.into().as_str() {
            "PENDING" => Ok(Self::Pending),
            "VERIFIED" => Ok(Self::Verified),
            "EXPIRED" => Ok(Self::Expired),
            "LOCKED" => Ok(Self::Locked),
            e => Err(IdentityDomainError::InvalidIdentityStatus(e.to_owned())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "PENDING",
            Self::Verified => "VERIFIED",
            Self::Expired => "EXPIRED",
            Self::Locked => "LOCKED",
        }
    }
}
