use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum IdentityDomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid identity status: {0}")]
    InvalidIdentityStatus(String),

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Invalid state transition")]
    InvalidStateTransition,

    #[error("Invalid timestamps: {0}")]
    InvalidTimestamps(String),
}
