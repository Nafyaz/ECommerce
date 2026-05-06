use crate::modules::shared::kernel::money::Currency;
use crate::modules::shared::kernel::money::errors::MoneyError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Money {
    amount: u64,
    currency: Currency,
}

impl Money {
    pub fn new(amount: u64, currency: Currency) -> Result<Self, MoneyError> {
        Ok(Self { amount, currency })
    }
}
