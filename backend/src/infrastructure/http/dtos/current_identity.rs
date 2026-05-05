use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

// TODO: Add roles. Or, roles will be added in application handlers??
#[derive(Clone, Debug)]
pub struct CurrentIdentity {
    pub identity_id: Uuid,
    // pub roles: Vec<String>,
}

impl<S> FromRequestParts<S> for CurrentIdentity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentIdentity>()
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))
    }
}
