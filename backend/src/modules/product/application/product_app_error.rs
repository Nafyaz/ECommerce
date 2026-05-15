use crate::modules::product::domain::ProductDomainError;
use crate::modules::product::ports::outbound::ProductRepositoryError;
use crate::modules::shared::MoneyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProductAppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("actor is not verified")]
    ActorNotVerified,

    #[error("vendor not found")]
    VendorNotFound,

    #[error("actor does not own this vendor")]
    VendorOwnershipMismatch,

    #[error("resource not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("{0} dependency unavailable")]
    DependencyUnavailable(&'static str),

    #[error("persistence unavailable")]
    PersistenceUnavailable,

    #[error("internal error")]
    Internal,
}

impl From<ProductDomainError> for ProductAppError {
    fn from(err: ProductDomainError) -> Self {
        match err {
            ProductDomainError::InvalidProductName(_) | ProductDomainError::InvalidProductPrice(_) => {
                Self::InvalidInput(err.to_string())
            }
            ProductDomainError::InvalidTimestamps => Self::Conflict(err.to_string()),
        }
    }
}

impl From<MoneyError> for ProductAppError {
    fn from(err: MoneyError) -> Self {
        Self::InvalidInput(err.to_string())
    }
}

impl ProductAppError {
    pub fn from_product_repository_error(err: ProductRepositoryError) -> Self {
        match err {
            ProductRepositoryError::NotFound => Self::NotFound,
            ProductRepositoryError::Conflict => Self::Conflict(err.to_string()),
            ProductRepositoryError::Unavailable => Self::PersistenceUnavailable,
            ProductRepositoryError::CorruptData(_) | ProductRepositoryError::Unexpected => Self::Internal,
        }
    }
}

impl From<ProductRepositoryError> for ProductAppError {
    fn from(err: ProductRepositoryError) -> Self {
        Self::from_product_repository_error(err)
    }
}
