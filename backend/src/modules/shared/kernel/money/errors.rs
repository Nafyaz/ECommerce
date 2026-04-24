use crate::modules::shared::AppError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MoneyError {
    #[error("Unsupported currency: {0}")]
    UnsupportedCurrency(String),
}

impl From<MoneyError> for AppError {
    fn from(error: MoneyError) -> Self {
        match error {
            MoneyError::UnsupportedCurrency(currency) => {
                AppError::Validation(format!("Unsupported currency: {}", currency))
            }
        }
    }
}
