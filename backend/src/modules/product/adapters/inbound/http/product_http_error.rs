use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::modules::product::application::{ProductAppError, ProductImageAppError};

#[derive(Debug, thiserror::Error)]
pub enum ProductHttpError {
    #[error("{0}")]
    Product(#[from] ProductAppError),

    #[error("{0}")]
    ProductImage(#[from] ProductImageAppError),
}

impl ProductHttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Product(error) => match error {
                ProductAppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
                ProductAppError::ActorNotVerified | ProductAppError::VendorOwnershipMismatch => StatusCode::FORBIDDEN,
                ProductAppError::VendorNotFound | ProductAppError::NotFound => StatusCode::NOT_FOUND,
                ProductAppError::Conflict(_) => StatusCode::CONFLICT,
                ProductAppError::DependencyUnavailable(_) | ProductAppError::PersistenceUnavailable => {
                    StatusCode::SERVICE_UNAVAILABLE
                }
                ProductAppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::ProductImage(error) => match error {
                ProductImageAppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
                ProductImageAppError::ActorNotVerified | ProductImageAppError::VendorOwnershipMismatch => {
                    StatusCode::FORBIDDEN
                }
                ProductImageAppError::ProductNotFound | ProductImageAppError::ImageNotFound => StatusCode::NOT_FOUND,
                ProductImageAppError::InvalidState
                | ProductImageAppError::DisplayOrderConflict
                | ProductImageAppError::Conflict(_) => StatusCode::CONFLICT,
                ProductImageAppError::DependencyUnavailable(_)
                | ProductImageAppError::StorageUnavailable
                | ProductImageAppError::PersistenceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                ProductImageAppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_code(&self) -> &'static str {
        match self.status_code() {
            StatusCode::BAD_REQUEST => "BAD_REQUEST",
            StatusCode::FORBIDDEN => "FORBIDDEN",
            StatusCode::NOT_FOUND => "NOT_FOUND",
            StatusCode::CONFLICT => "CONFLICT",
            StatusCode::SERVICE_UNAVAILABLE => "SERVICE_UNAVAILABLE",
            _ => "INTERNAL_ERROR",
        }
    }
}

impl IntoResponse for ProductHttpError {
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
