use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::modules::identity::application::IdentityAppError;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct IdentityHttpError(#[from] IdentityAppError);

impl IdentityHttpError {
    fn status_code(&self) -> StatusCode {
        match &self.0 {
            IdentityAppError::InvalidInput(_) | IdentityAppError::InvalidOtp => StatusCode::BAD_REQUEST,
            IdentityAppError::InvalidCredentials | IdentityAppError::EmailNotVerified => StatusCode::UNAUTHORIZED,
            IdentityAppError::IdentityNotFound => StatusCode::NOT_FOUND,
            IdentityAppError::VerifiedIdentityAlreadyExists
            | IdentityAppError::NoActiveOtp
            | IdentityAppError::Conflict(_) => StatusCode::CONFLICT,
            IdentityAppError::DependencyUnavailable(_) | IdentityAppError::PersistenceUnavailable => {
                StatusCode::SERVICE_UNAVAILABLE
            }
            IdentityAppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self.status_code() {
            StatusCode::BAD_REQUEST => "BAD_REQUEST",
            StatusCode::UNAUTHORIZED => "UNAUTHORIZED",
            StatusCode::NOT_FOUND => "NOT_FOUND",
            StatusCode::CONFLICT => "CONFLICT",
            StatusCode::SERVICE_UNAVAILABLE => "SERVICE_UNAVAILABLE",
            _ => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for IdentityHttpError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let code = self.error_code();

        let body = json!({
            "success": false,
            "error": {
                "code": code,
                "message": self.to_string(),
            }
        });

        (status, Json(body)).into_response()
    }
}
