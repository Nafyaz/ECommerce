use crate::modules::shared::kernel::money::errors::CurrencyError;
use std::fmt;
use std::str::FromStr;

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

    pub fn scale(self) -> u32 {
        match self {
            Currency::JPY | Currency::KRW => 0,
            Currency::KWD => 3,
            _ => 2,
        }
    }
}

impl FromStr for Currency {
    type Err = CurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "BDT" => Ok(Currency::BDT),
            "INR" => Ok(Currency::INR),
            "AUD" => Ok(Currency::AUD),
            "CAD" => Ok(Currency::CAD),
            "CHF" => Ok(Currency::CHF),
            "CNY" => Ok(Currency::CNY),
            "HKD" => Ok(Currency::HKD),
            "SGD" => Ok(Currency::SGD),
            "SEK" => Ok(Currency::SEK),
            "NOK" => Ok(Currency::NOK),
            "DKK" => Ok(Currency::DKK),
            "NZD" => Ok(Currency::NZD),
            "KRW" => Ok(Currency::KRW),
            "TRY" => Ok(Currency::TRY),
            "RUB" => Ok(Currency::RUB),
            "BRL" => Ok(Currency::BRL),
            "MXN" => Ok(Currency::MXN),
            "AED" => Ok(Currency::AED),
            "SAR" => Ok(Currency::SAR),
            "KWD" => Ok(Currency::KWD),

            other => Err(CurrencyError::Unsupported(other.to_string())),
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}
