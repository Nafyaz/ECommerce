use crate::modules::identity::{IdentityError, IdentityId};

pub enum AuthError {
    AuthenticationFailed,
}

pub trait AuthenticatorPort {
    fn authenticate(&self, token: &str) -> Result<IdentityId, AuthError>;
}
