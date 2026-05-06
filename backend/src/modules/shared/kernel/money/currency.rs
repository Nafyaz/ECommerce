use crate::modules::shared::kernel::money::errors::MoneyError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Currency {
    BDT,
    EUR,
    USD,
}

impl Currency {
    pub fn new(currency: String) -> Result<Self, MoneyError> {
        match currency.as_str() {
            "bdt" => Ok(Currency::BDT),
            "eur" => Ok(Currency::EUR),
            "usd" => Ok(Currency::USD),
            _ => Err(MoneyError::UnsupportedCurrency(currency)),
        }
    }
}
