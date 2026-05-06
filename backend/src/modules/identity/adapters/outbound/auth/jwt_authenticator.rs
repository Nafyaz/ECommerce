use crate::modules::identity::ports::outbound::{AuthError, AuthenticatorPort};
use crate::modules::identity::{IdentityId, TokenServicePort};
use std::sync::Arc;

pub struct JwtAuthenticator {
    token_service: Arc<dyn TokenServicePort>,
}

impl JwtAuthenticator {
    pub fn new(token_service: Arc<dyn TokenServicePort>) -> Self {
        Self { token_service }
    }
}

impl AuthenticatorPort for JwtAuthenticator {
    fn authenticate(&self, token: &str) -> Result<IdentityId, AuthError> {
        tracing::trace!("inside authenticator");

        let claims = self
            .token_service
            .validate_token(token)
            .map_err(|_| AuthError::AuthenticationFailed)?;

        tracing::trace!("Token validated: {:?}", claims);

        Ok(IdentityId::from_uuid(claims.sub))
    }
}
