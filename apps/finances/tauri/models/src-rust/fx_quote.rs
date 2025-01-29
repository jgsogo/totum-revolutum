use std::ops::Mul;

use super::google_type;

use crate::google_type::CurrencyCode;

use crate::errors::OperationError;
use crate::{Error, Result};
use bigdecimal::BigDecimal;
use proto_wrapper::ProtoWrapper;

/// FxQuote pair
///
/// **Example**
///
/// Currencies are quoted in relation to another currency. For example, when we refer to the exchange rate
/// of the euro (the currency of the European Union) to the U.S. dollar we quote the relationship, or
/// exchange rate, as EUR/USD.
///
/// The first currency in the quotation – in this case the euro – is represented by its three-letter
/// symbol, EUR. This is known as the named or base currency.
///
/// The second currency – in this case the U.S. dollar, shown by its three-letter symbol, USD – is known
/// as the terms or quote currency.
///
/// When trading FX, the trading action is applied to the base, or first, currency in the currency pair.
/// So, if you purchase the EUR/USD at 1.1250, you would receive one unit of the euro (EUR) in exchange
/// for a payment of 1.1250 U.S. dollars (USD).
pub struct FxQuotePair {
    /// named or base currency
    base: CurrencyCode,

    /// terms or quote currency
    quote: CurrencyCode,
}

impl FxQuotePair {
    pub fn new(base: CurrencyCode, quote: CurrencyCode) -> Result<Self> {
        if base == quote {
            Err(Error::InvalidFxQuotePair)
        } else {
            Ok(Self { base, quote })
        }
    }

    pub fn base(&self) -> CurrencyCode {
        self.base
    }

    pub fn quote(&self) -> CurrencyCode {
        self.quote
    }
}

impl std::fmt::Display for FxQuotePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

/// A wrapper over the FxQuote protobuf.
#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct FxQuote(crate::protos::finances_app_models::FxQuote);

impl FxQuote {
    pub fn new(pair: FxQuotePair, date_value: google_type::Date, quote: google_type::Decimal) -> Self {
        Self(crate::protos::finances_app_models::FxQuote {
            date_value: Some(date_value.into()),
            base_ccy_code: pair.base.to_string(),
            quote_ccy_code: pair.quote.to_string(),
            quote: Some(quote.into()),
        })
    }

    pub fn date_value(&self) -> Result<&google_type::Date> {
        self.0
            .date_value
            .as_ref()
            .map(google_type::Date::new_ref)
            .ok_or(Error::MissingRequiredField("date_value".to_string()))
    }

    pub fn quote(&self) -> Result<&google_type::Decimal> {
        self.0
            .quote
            .as_ref()
            .map(google_type::Decimal::new_ref)
            .ok_or(Error::MissingRequiredField("fx".to_string()))
    }

    pub fn fx_pair(&self) -> Result<FxQuotePair> {
        let base = CurrencyCode::new(&self.0.base_ccy_code)?;
        let quote = CurrencyCode::new(&self.0.quote_ccy_code)?;
        Ok(FxQuotePair { base, quote })
    }

    /// Takes the reciprocal (inverse) of the Fx quote, 1/x.
    pub fn recip(self) -> Result<Self> {
        let quote: Option<BigDecimal> = self
            .0
            .quote
            .map(google_type::Decimal::from_proto)
            .map(|v| v.value())
            .transpose()?;
        let inv_quote = quote.map(|v| 1 / v);
        let inv_quote = inv_quote.map(google_type::Decimal::new);

        Ok(Self(crate::protos::finances_app_models::FxQuote {
            date_value: self.0.date_value,
            base_ccy_code: self.0.quote_ccy_code,
            quote_ccy_code: self.0.base_ccy_code,
            quote: inv_quote.map(|v| v.into()),
        }))
    }
}

impl Mul<&FxQuote> for &google_type::Money {
    type Output = Result<google_type::Money>;

    fn mul(self, rhs: &FxQuote) -> Self::Output {
        let money_ccy = self.currency_code()?;
        let fx_pair = rhs.fx_pair()?;

        if money_ccy == fx_pair.base {
            let amount = self.amount() * rhs.quote()?.value()?;
            Ok(google_type::Money::new(amount, fx_pair.quote)?)
        } else if money_ccy == fx_pair.quote {
            let amount = self.amount() / rhs.quote()?.value()?;
            Ok(google_type::Money::new(amount, fx_pair.base)?)
        } else {
            Err(OperationError::FxQuoteMismatch.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use google_type::Money;

    use super::*;

    #[test]
    fn fx_pair_display() {
        let pair = FxQuotePair::new(CurrencyCode::EUR, CurrencyCode::USD).unwrap();
        assert_eq!(pair.to_string(), "EUR/USD");
    }

    #[test]
    fn mul_money() {
        let eur_money = {
            let amount = BigDecimal::from_str("2.00").unwrap();
            Money::new(amount, CurrencyCode::EUR).unwrap()
        };

        let quote = {
            let pair = FxQuotePair::new(CurrencyCode::EUR, CurrencyCode::USD).unwrap();
            let date_value = google_type::Date::new(2025, 1, 24).unwrap();
            let quote = google_type::Decimal::new(BigDecimal::from_str("0.5").unwrap());
            FxQuote::new(pair, date_value, quote)
        };

        let usd_money = (&eur_money * &quote).unwrap();
        assert_eq!(usd_money.amount().to_string(), "1");
        assert_eq!(usd_money.currency_code().unwrap(), CurrencyCode::USD);
    }

    #[test]
    fn recip() {
        let quote = {
            let pair = FxQuotePair::new(CurrencyCode::EUR, CurrencyCode::USD).unwrap();
            let date_value = google_type::Date::new(2025, 1, 24).unwrap();
            let quote = google_type::Decimal::new(BigDecimal::from_str("0.5").unwrap());
            FxQuote::new(pair, date_value, quote)
        };
        assert_eq!(quote.fx_pair().unwrap().to_string(), "EUR/USD");
        let quote_rate: BigDecimal = quote.quote().unwrap().try_into().unwrap();
        assert_eq!(quote_rate.to_string(), "0.5");

        let recip = quote.recip().unwrap();
        assert_eq!(recip.fx_pair().unwrap().to_string(), "USD/EUR");
        let recip_quote_rate: BigDecimal = recip.quote().unwrap().try_into().unwrap();
        assert_eq!(recip_quote_rate.to_string(), "2");
    }
}
