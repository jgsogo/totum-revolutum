
use crate::errors::ConversionError;

/// Currency code, the enum variants follow [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyCode {
    #[allow(clippy::upper_case_acronyms)]
    EUR,
    #[allow(clippy::upper_case_acronyms)]
    USD,
}

/// Prints the [`CurrencyCode`] as a [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html) string.
impl std::fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyCode::EUR => write!(f, "EUR"),
            CurrencyCode::USD => write!(f, "USD"),
        }
    }
}

impl CurrencyCode {
    pub fn new(ccy: &str) -> Result<Self, crate::errors::ConversionError> {
        match ccy {
            "EUR" => Ok(CurrencyCode::EUR),
            "USD" => Ok(CurrencyCode::USD),
            _ => Err(ConversionError::InvalidCurrencyCode(ccy.to_string())),
        }
    }
}

/// Tries to create a [`CurrencyCode`] from a string following [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html) rule.
impl TryFrom<&str> for CurrencyCode {
    type Error = crate::errors::ConversionError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        CurrencyCode::new(value)
    }
}
