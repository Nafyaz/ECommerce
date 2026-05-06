use crate::modules::identity::{IdentityError, IdentityId};

pub enum AuthError {
    AuthenticationFailed,
}

pub trait AuthenticatorPort: Send + Sync {
    fn authenticate(&self, token: &str) -> Result<IdentityId, AuthError>;
}
