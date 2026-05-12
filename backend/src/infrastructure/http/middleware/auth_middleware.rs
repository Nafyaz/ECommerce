use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::identity::ports::outbound::AuthenticatorPort;
use crate::modules::shared::AppError;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub authenticator: Arc<dyn AuthenticatorPort>,
}

impl AuthState {
    pub fn new(authenticator: Arc<dyn AuthenticatorPort>) -> Self {
        Self { authenticator }
    }
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid header encoding".into()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Malformed Bearer token".into()))?;

    let identity_id = state
        .authenticator
        .authenticate(token)
        .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

    let current_identity = CurrentIdentity {
        identity_id: identity_id.as_uuid().to_owned(),
    };

    request.extensions_mut().insert(current_identity);

    let response = next.run(request).await;

    Ok(response)
}
