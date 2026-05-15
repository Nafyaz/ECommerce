use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ProductHttpError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl IntoResponse for ProductHttpError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            ProductHttpError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            ProductHttpError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            ProductHttpError::Conflict(_) => (StatusCode::CONFLICT, "CONFLICT"),
            ProductHttpError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            ProductHttpError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            ProductHttpError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
        };

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
