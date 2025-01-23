use bigdecimal::BigDecimal;

use crate::traits::ProtoWrapperRef;

use super::money::NANO_VALUE;
use super::CurrencyCode;

// use super::{money::{MoneyTrait, NANO_VALUE}, CurrencyCode};

pub struct MoneyRef<'a>(&'a crate::protos::google::r#type::Money);

impl<'a> MoneyRef<'a> {
    #[must_use]
    pub fn amount(&self) -> BigDecimal {
        let amount = BigDecimal::from(self.0.units);
        let fractional = BigDecimal::from(self.0.nanos) / NANO_VALUE;
        amount + fractional
    }

    pub fn currency_code(&self) -> Result<CurrencyCode, crate::errors::ConversionError> {
        CurrencyCode::new(&self.0.currency_code)
    }
}

impl<'a> ProtoWrapperRef<'a, crate::protos::google::r#type::Money> for MoneyRef<'a> {
    fn new(proto: &'a crate::protos::google::r#type::Money) -> Self {
        Self(proto)
    }
}

impl<'a> From<MoneyRef<'a>> for &'a crate::protos::google::r#type::Money {
    fn from(val: MoneyRef<'a>) -> Self {
        val.0
    }
}

impl<'a> From<&'a crate::protos::google::r#type::Money> for MoneyRef<'a> {
    fn from(v: &'a crate::protos::google::r#type::Money) -> Self {
        Self(v)
    }
}

impl<'a> Into<crate::protos::google::r#type::Money> for MoneyRef<'a> {
    fn into(self) -> crate::protos::google::r#type::Money {
        self.0.clone()
    }
}

// impl<'a> MoneyTrait for MoneyRef<'a> {
//     #[must_use]
//     fn amount(&self) -> BigDecimal {
//         let amount = BigDecimal::from(self.0.units);
//         let fractional = BigDecimal::from(self.0.nanos) / NANO_VALUE;
//         amount + fractional
//     }

//     fn currency_code(&self) -> Result<CurrencyCode, crate::errors::ConversionError> {
//         CurrencyCode::new(&self.0.currency_code)
//     }
// }
