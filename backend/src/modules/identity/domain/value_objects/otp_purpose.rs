use crate::modules::identity::domain::OtpDomainError;
use crate::modules::identity::domain::value_objects::TokenType;

//TODO: Implement phone verification, email change, password change, delete account
// TODO: Implement MFA for login
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OtpPurpose {
    Registration,
    EmailVerification,
    PhoneVerification,
    PasswordReset,
    EmailChange,
    PasswordChange,
    DeleteAccount,
}

impl OtpPurpose {
    pub fn from_str(otp_purpose: impl Into<String>) -> Result<Self, OtpDomainError> {
        match otp_purpose.into().as_str() {
            "REGISTRATION" => Ok(Self::Registration),
            "EMAIL_VERIFICATION" => Ok(Self::EmailVerification),
            "PHONE_VERIFICATION" => Ok(Self::PhoneVerification),
            "PASSWORD_RESET" => Ok(Self::PasswordReset),
            "EMAIL_CHANGE" => Ok(Self::EmailChange),
            "PASSWORD_CHANGE" => Ok(Self::PasswordChange),
            "DELETE_ACCOUNT" => Ok(Self::DeleteAccount),
            op => Err(OtpDomainError::InvalidOtpPurpose(op.into())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Registration => "REGISTRATION",
            Self::EmailVerification => "EMAIL_VERIFICATION",
            Self::PhoneVerification => "PHONE_VERIFICATION",
            Self::PasswordReset => "PASSWORD_RESET",
            Self::EmailChange => "EMAIL_CHANGE",
            Self::PasswordChange => "PASSWORD_CHANGE",
            Self::DeleteAccount => "DELETE_ACCOUNT",
        }
    }

    pub fn issuing_token_after_verification(&self) -> Option<&TokenType> {
        match self {
            Self::EmailChange => Some(&TokenType::EmailChange),
            Self::PasswordChange => Some(&TokenType::PasswordChange),
            Self::DeleteAccount => Some(&TokenType::DeleteAccount),
            _ => None,
        }
    }
}
