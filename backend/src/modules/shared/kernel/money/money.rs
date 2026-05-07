use crate::modules::shared::kernel::money::Currency;
use crate::modules::shared::kernel::money::errors::MoneyError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    amount_minor: i64,
    currency: Currency,
}

impl Money {
    pub fn new(amount_minor: i64, currency: Currency) -> Result<Self, MoneyError> {
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
