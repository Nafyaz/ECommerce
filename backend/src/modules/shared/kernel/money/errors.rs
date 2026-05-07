use crate::modules::shared::{AppError, Currency};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MoneyError {
    #[error("Unsupported currency: {0}")]
    UnsupportedCurrency(String),

    #[error("Invalid amount: {0}")]
    InvalidAmount(i64),

    #[error("currency mismatch: {left:?} vs {right:?}")]
    CurrencyMismatch { left: Currency, right: Currency },

    #[error("overflow")]
    Overflow,
}

impl From<MoneyError> for AppError {
    fn from(error: MoneyError) -> Self {
        match error {
            MoneyError::UnsupportedCurrency(currency) => {
                AppError::Validation(format!("Unsupported currency: {}", currency))
            }
            MoneyError::InvalidAmount(amount) => AppError::Validation(format!("Invalid amount: {}", amount)),

            MoneyError::CurrencyMismatch { left, right } => {
                AppError::Validation(format!("Currency mismatch: {} vs {}", left, right))
            }
            MoneyError::Overflow => AppError::Validation("Overflow".to_string()),
        }
    }
}
