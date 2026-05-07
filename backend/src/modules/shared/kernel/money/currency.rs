use crate::modules::shared::kernel::money::errors::MoneyError;
use std::fmt;

// TODO: Learn ISO rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    BDT,
    INR,
    AUD,
    CAD,
    CHF,
    CNY,
    HKD,
    SGD,
    SEK,
    NOK,
    DKK,
    NZD,
    KRW,
    TRY,
    RUB,
    BRL,
    MXN,
    AED,
    SAR,
    KWD,
}
impl Currency {
    pub fn code(self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::BDT => "BDT",
            Currency::INR => "INR",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::HKD => "HKD",
            Currency::SGD => "SGD",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::NZD => "NZD",
            Currency::KRW => "KRW",
            Currency::TRY => "TRY",
            Currency::RUB => "RUB",
            Currency::BRL => "BRL",
            Currency::MXN => "MXN",
            Currency::AED => "AED",
            Currency::SAR => "SAR",
            Currency::KWD => "KWD",
        }
    }

    pub fn exponent(self) -> u32 {
        match self {
            Currency::JPY | Currency::KRW => 0,
            Currency::KWD => 3,
            _ => 2,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
