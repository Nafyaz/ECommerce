use crate::modules::product::domain::ProductDomainError;
use crate::modules::product::ports::outbound::ProductRepositoryError;

impl From<ProductDomainError> for ProductAppError {
    fn from(err: ProductDomainError) -> Self {
        match err {
            ProductDomainError::InvalidProductName(_) => Self::InvalidInput(err.to_string()),
            ProductDomainError::ReconstitutionError(_) => Self::Conflict(err.to_string()),
        }
    }
}

impl From<ProductRepositoryError> for ProductAppError {
    fn from(err: ProductRepositoryError) -> Self {
        match err {
            ProductRepositoryError::NotFound => Self::NotFound,
            ProductRepositoryError::OperationFailed => Self::Internal,
        }
    }
}
