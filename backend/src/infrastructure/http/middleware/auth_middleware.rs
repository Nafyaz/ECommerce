use crate::modules::identity::{TokenServicePort, UserDomainError};
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
        .ok_or(UserDomainError::MissingToken)?;

    let token = auth_header.to_str().map_err(|_| UserDomainError::InvalidToken)?;

    let claim = state.token_service.validate_token(token)?;

    request.extensions_mut().insert(claim);

    let response = next.run(request).await;

    Ok(response)
}
