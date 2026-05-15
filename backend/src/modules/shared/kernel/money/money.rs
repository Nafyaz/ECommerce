use crate::modules::shared::kernel::money::currency::Currency;
use crate::modules::shared::kernel::money::errors::MoneyError;
use crate::modules::shared::kernel::money::rounding_mode::RoundingMode;
use rust_decimal::{Decimal, RoundingStrategy, prelude::ToPrimitive};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount_minor: i64, currency: impl Into<String>) -> Result<Self, MoneyError> {
        let currency = Currency::from_str(currency.into().as_str())?;

        Ok(Self { amount_minor, currency })
    }

    pub fn amount_minor(&self) -> i64 {
        self.amount_minor
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn checked_add(self, rhs: Self) -> Result<Self, MoneyError> {
        if self.currency != rhs.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency,
                right: rhs.currency,
            });
        }

        let amount = self
            .amount_minor
            .checked_add(rhs.amount_minor)
            .ok_or(MoneyError::Overflow)?;

        Ok(Self {
            amount_minor: amount,
            currency: self.currency,
        })
    }

    pub fn checked_sub(self, rhs: Self) -> Result<Self, MoneyError> {
        if self.currency != rhs.currency {
            return Err(MoneyError::CurrencyMismatch {
                left: self.currency,
                right: rhs.currency,
            });
        }

        let amount = self
            .amount_minor
            .checked_sub(rhs.amount_minor)
            .ok_or(MoneyError::Overflow)?;

        Ok(Self {
            amount_minor: amount,
            currency: self.currency,
        })
    }

    pub fn multiply(self, multiplier: Decimal, rounding: RoundingMode) -> Result<Self, MoneyError> {
        let amount = Decimal::from(self.amount_minor);

        let result = amount * multiplier;

        let rounded = match rounding {
            RoundingMode::HalfUp => result.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero),
            RoundingMode::HalfEven => result.round_dp_with_strategy(0, RoundingStrategy::MidpointNearestEven),
            RoundingMode::Floor => result.floor(),
            RoundingMode::Ceiling => result.ceil(),
        };

        let amount_minor = rounded.to_i64().ok_or(MoneyError::Overflow)?;

        Ok(Self {
            amount_minor,
            currency: self.currency,
        })
    }

    pub fn divide(self, divisor: Decimal, rounding: RoundingMode) -> Result<Self, MoneyError> {
        if divisor.is_zero() {
            return Err(MoneyError::DivisionByZero);
        }

        let amount = Decimal::from(self.amount_minor);

        let result = amount / divisor;

        let rounded = match rounding {
            RoundingMode::HalfUp => result.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero),
            RoundingMode::HalfEven => result.round_dp_with_strategy(0, RoundingStrategy::MidpointNearestEven),
            RoundingMode::Floor => result.floor(),
            RoundingMode::Ceiling => result.ceil(),
        };

        let amount_minor = rounded.to_i64().ok_or(MoneyError::Overflow)?;

        Ok(Self {
            amount_minor,
            currency: self.currency,
        })
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = self.currency.scale();

        if scale == 0 {
            return write!(f, "{} {}", self.currency, self.amount_minor);
        }

        let divisor = 10_i64.pow(scale);

        let abs = self.amount_minor.abs();

        let major = abs / divisor;
        let minor = abs % divisor;

        if self.amount_minor < 0 {
            write!(
                f,
                "{} -{}.{:0width$}",
                self.currency,
                major,
                minor,
                width = scale as usize
            )
        } else {
            write!(
                f,
                "{} {}.{:0width$}",
                self.currency,
                major,
                minor,
                width = scale as usize
            )
        }
    }
}
