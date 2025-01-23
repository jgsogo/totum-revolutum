use bigdecimal::BigDecimal;

use crate::traits::ProtoWrapperRef;

use super::money::NANO_VALUE;
use super::CurrencyCode;

// use super::{money::{MoneyTrait, NANO_VALUE}, CurrencyCode};

#[repr(transparent)]
pub struct MoneyRef(crate::protos::google::r#type::Money);

impl MoneyRef {
    pub fn new(proto: &crate::protos::google::r#type::Money) -> &Self {
        let v = unsafe { &*(proto as *const crate::protos::google::r#type::Money as *const Self) };
        v
    }

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

impl ProtoWrapperRef<crate::protos::google::r#type::Money> for MoneyRef {
    fn new(proto: &crate::protos::google::r#type::Money) -> &Self {
        MoneyRef::new(proto)
    }
    // fn new(proto: &'a crate::protos::google::r#type::Money) -> Self {
    //     Self(proto)
    // }
}

// impl<'a> From<MoneyRef<'a>> for &'a crate::protos::google::r#type::Money {
//     fn from(val: MoneyRef<'a>) -> Self {
//         val.0
//     }
// }

// impl<'a> From<&'a crate::protos::google::r#type::Money> for MoneyRef<'a> {
//     fn from(v: &'a crate::protos::google::r#type::Money) -> Self {
//         Self(v)
//     }
// }

impl Into<crate::protos::google::r#type::Money> for MoneyRef {
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
