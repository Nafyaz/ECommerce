use crate::modules::shared::kernel::money::currency::Currency;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CurrencyError {
    #[error("Unsupported currency: {0}")]
    Unsupported(String),
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

    #[error("{0}")]
    CurrencyError(#[from] CurrencyError),
}
