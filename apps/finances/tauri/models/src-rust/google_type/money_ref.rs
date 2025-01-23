use bigdecimal::BigDecimal;

use bigdecimal::ToPrimitive;
use bigdecimal::Zero;

use crate::traits::ProtoWrapperRef;

const NANO_EXP: usize = 9;
pub(crate) const NANO_VALUE: u32 = 1_000_000_000;
use super::CurrencyCode;
use prost::Message;
// use super::{money::{MoneyTrait, NANO_VALUE}, CurrencyCode};

#[repr(transparent)]
pub struct MoneyRef(crate::protos::google::r#type::Money);

impl MoneyRef {
    pub fn new(amount: BigDecimal, ccy: CurrencyCode) -> Result<Self, crate::errors::ConversionError> {
        if amount.is_zero() {
            return Ok(Self(crate::protos::google::r#type::Money {
                currency_code: ccy.to_string(),
                units: i64::zero(),
                nanos: i32::zero(),
            }));
        }

        let (int_val, scale) = amount.as_bigint_and_scale();
        let (sign, mut digits) = int_val.to_radix_be(10);
        let (integer_part, fractional_part) = if scale <= 0 {
            let scale: usize = scale.unsigned_abs().try_into()?;
            digits.extend(std::iter::repeat(0).take(scale));
            let integer_part = num_bigint::BigInt::from_radix_be(sign, &digits, 10)
                .ok_or(crate::errors::ConversionError::BigIntFromRadix10Error)?;
            (integer_part, num_bigint::BigInt::zero())
        } else {
            let scale: usize = scale.try_into()?;
            let idx = digits.len() - scale;
            let integer_part = num_bigint::BigInt::from_radix_be(sign, &digits[0..idx], 10)
                .ok_or(crate::errors::ConversionError::BigIntFromRadix10Error)?;
            let mut fractional_digits = digits[idx..].to_vec();
            fractional_digits.resize(NANO_EXP, u8::zero());
            let fractional_part = num_bigint::BigInt::from_radix_be(sign, &fractional_digits, 10)
                .ok_or(crate::errors::ConversionError::BigIntFromRadix10Error)?;
            (integer_part, fractional_part)
        };

        Ok(Self(crate::protos::google::r#type::Money {
            currency_code: ccy.to_string(),
            units: integer_part
                .to_i64()
                .ok_or(crate::errors::ConversionError::I64Overflow(integer_part))?,
            nanos: fractional_part
                .to_i32()
                .ok_or(crate::errors::ConversionError::I32Overflow(fractional_part))?,
        }))
    }

    pub fn new_ref(proto: &crate::protos::google::r#type::Money) -> &Self {
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
        MoneyRef::new_ref(proto)
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

impl From<MoneyRef> for crate::protos::google::r#type::Money {
    fn from(val: MoneyRef) -> Self {
        val.0
    }
}

impl From<&MoneyRef> for crate::protos::google::r#type::Money {
    fn from(val: &MoneyRef) -> Self {
        val.0.clone()
    }
}

impl AsRef<MoneyRef> for MoneyRef {
    fn as_ref(&self) -> &MoneyRef {
        MoneyRef::new_ref(&self.0)
    }
}

impl TryFrom<Vec<u8>> for MoneyRef {
    type Error = crate::errors::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::google::r#type::Money::decode(&*value)?))
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
