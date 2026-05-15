use crate::modules::product::domain::ProductImageDomainError;
use crate::modules::product::ports::outbound::{
    ObjectStorageError, ProductImageRepositoryError, ProductRepositoryError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductImageAppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("actor is not verified")]
    ActorNotVerified,

    #[error("actor does not own this vendor")]
    VendorOwnershipMismatch,

    #[error("product not found")]
    ProductNotFound,

    #[error("image not found")]
    ImageNotFound,

    #[error("invalid image state")]
    InvalidState,

    #[error("display order is already used for this product")]
    DisplayOrderConflict,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("{0} dependency unavailable")]
    DependencyUnavailable(&'static str),

    #[error("storage unavailable")]
    StorageUnavailable,

    #[error("persistence unavailable")]
    PersistenceUnavailable,

    #[error("internal error")]
    Internal,
}

impl From<ProductImageDomainError> for ProductImageAppError {
    fn from(err: ProductImageDomainError) -> Self {
        match err {
            ProductImageDomainError::InvalidContentType(_)
            | ProductImageDomainError::InvalidSize { .. }
            | ProductImageDomainError::InvalidDisplayOrder(_)
            | ProductImageDomainError::InvalidStatus(_)
            | ProductImageDomainError::InvalidObjectKey(_) => Self::InvalidInput(err.to_string()),
            ProductImageDomainError::InvalidStateTransition => Self::InvalidState,
            ProductImageDomainError::InvalidTimestamps(_) => Self::Conflict(err.to_string()),
        }
    }
}

impl ProductImageAppError {
    pub fn from_product_repository_error(err: ProductRepositoryError) -> Self {
        match err {
            ProductRepositoryError::NotFound => Self::ProductNotFound,
            ProductRepositoryError::Conflict => Self::Conflict(err.to_string()),
            ProductRepositoryError::Unavailable => Self::PersistenceUnavailable,
            ProductRepositoryError::CorruptData(_) | ProductRepositoryError::Unexpected => Self::Internal,
        }
    }
}

impl From<ProductImageRepositoryError> for ProductImageAppError {
    fn from(err: ProductImageRepositoryError) -> Self {
        match err {
            ProductImageRepositoryError::NotFound => Self::ImageNotFound,
            ProductImageRepositoryError::ProductNotFound => Self::ProductNotFound,
            ProductImageRepositoryError::DisplayOrderConflict => Self::DisplayOrderConflict,
            ProductImageRepositoryError::ObjectKeyConflict => Self::Conflict(err.to_string()),
            ProductImageRepositoryError::Unavailable => Self::PersistenceUnavailable,
            ProductImageRepositoryError::CorruptData(_) | ProductImageRepositoryError::Unexpected => Self::Internal,
        }
    }
}

impl From<ObjectStorageError> for ProductImageAppError {
    fn from(err: ObjectStorageError) -> Self {
        match err {
            ObjectStorageError::NotFound => Self::ImageNotFound,
            ObjectStorageError::InvalidRequest => Self::InvalidInput(err.to_string()),
            ObjectStorageError::Unavailable | ObjectStorageError::StorageFailure(_) => Self::StorageUnavailable,
            ObjectStorageError::Unexpected => Self::Internal,
        }
    }
}
