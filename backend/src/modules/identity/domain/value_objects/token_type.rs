#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Access,
    Refresh,
    PasswordChange,
    EmailChange,
    DeleteAccount,
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Access => "ACCESS",
            Self::Refresh => "REFRESH",
            Self::PasswordChange => "PASSWORD_CHANGE",
            Self::EmailChange => "EMAIL_CHANGE",
            Self::DeleteAccount => "DELETE_ACCOUNT",
        }
    }
}
