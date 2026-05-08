use crate::modules::shared::{AppError, Currency};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CurrencyError {
    #[error("Unsupported currency: {0}")]
    Unsupported(String),
}

impl From<CurrencyError> for AppError {
    fn from(error: CurrencyError) -> Self {
        match error {
            CurrencyError::Unsupported(currency) => AppError::Validation(format!("Unsupported currency: {}", currency)),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum MoneyError {
    #[error("Invalid amount: {0}")]
    InvalidAmount(i64),

    #[error("currency mismatch: {left:?} vs {right:?}")]
    CurrencyMismatch { left: Currency, right: Currency },

    #[error("overflow")]
    Overflow,

    #[error("division by zero")]
    DivisionByZero,
}

impl From<MoneyError> for AppError {
    fn from(error: MoneyError) -> Self {
        match error {
            MoneyError::InvalidAmount(amount) => AppError::Validation(format!("Invalid amount: {}", amount)),

            MoneyError::CurrencyMismatch { left, right } => {
                AppError::Validation(format!("Currency mismatch: {} vs {}", left, right))
            }
            MoneyError::Overflow => AppError::Validation("Overflow".to_string()),
            MoneyError::DivisionByZero => AppError::Validation("Division by zero".to_string()),
        }
    }
}
