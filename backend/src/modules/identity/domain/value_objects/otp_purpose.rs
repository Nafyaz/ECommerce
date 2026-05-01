use crate::modules::identity::IdentityError;

//TODO: Implement phone verification, login, email change, password change, delete account
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OtpPurpose {
    EmailVerification,
    PhoneVerification,
    Login,
    PasswordReset,
    EmailChange,
    PasswordChange,
    DeleteAccount,
}

impl OtpPurpose {
    pub fn from_str(otp_purpose: impl Into<String>) -> Result<Self, IdentityError> {
        match otp_purpose.into().as_str() {
            "EMAIL_VERIFICATION" => Ok(Self::EmailVerification),
            "PHONE_VERIFICATION" => Ok(Self::PhoneVerification),
            "LOGIN" => Ok(Self::Login),
            "PASSWORD_RESET" => Ok(Self::PasswordReset),
            "EMAIL_CHANGE" => Ok(Self::EmailChange),
            "PASSWORD_CHANGE" => Ok(Self::PasswordChange),
            "DELETE_ACCOUNT" => Ok(Self::DeleteAccount),
            op => Err(IdentityError::InvalidOtpPurpose(op.into())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::EmailVerification => "EMAIL_VERIFICATION",
            Self::PhoneVerification => "PHONE_VERIFICATION",
            Self::Login => "LOGIN",
            Self::PasswordReset => "PASSWORD_RESET",
            Self::EmailChange => "EMAMAIL_CHANGEIL_CHANGE",
            Self::PasswordChange => "PASSWORD_CHANGE",
            Self::DeleteAccount => "DELETE_ACCOUNT",
        }
    }
}
