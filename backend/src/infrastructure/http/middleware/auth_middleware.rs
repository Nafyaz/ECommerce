use crate::infrastructure::http::dtos::CurrentIdentity;
use crate::modules::identity::{IdentityDomainError, TokenServiceError, TokenServicePort};
use crate::modules::shared::AppError;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub token_service: Arc<dyn TokenServicePort>,
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(IdentityDomainError::from(TokenServiceError::MissingToken))?;

    let auth_header = auth_header
        .to_str()
        .map_err(|_| IdentityDomainError::from(TokenServiceError::InvalidToken))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(IdentityDomainError::from(TokenServiceError::InvalidToken))?;

    let claim = state
        .token_service
        .validate_token(token)
        .map_err(|_| IdentityDomainError::from(TokenServiceError::InvalidToken))?;

    let current_identity = CurrentIdentity { id: claim.sub };

    request.extensions_mut().insert(current_identity);

    let response = next.run(request).await;

    Ok(response)
}
