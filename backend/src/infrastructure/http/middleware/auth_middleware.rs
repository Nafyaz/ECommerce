use crate::infrastructure::http::dtos::CurrentUser;
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

    let auth_header = auth_header.to_str().map_err(|_| UserDomainError::InvalidToken)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(UserDomainError::InvalidToken)?;

    let claim = state.token_service.validate_token(token)?;

    let current_user = CurrentUser { id: claim.sub };

    request.extensions_mut().insert(current_user);

    let response = next.run(request).await;

    Ok(response)
}
