use std::ops::Deref;
use std::ops::Mul;

use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;

use super::CurrencyCode;
use super::MoneyRef;
use crate::traits::{ProtoWrapper, ProtoWrapperWithRef};
use prost::Message;

const NANO_EXP: usize = 9;
pub(crate) const NANO_VALUE: u32 = 1_000_000_000;

// pub trait MoneyTrait {
//     #[must_use]
//     fn amount(&self) -> BigDecimal;

//     fn currency_code(&self) -> Result<CurrencyCode, crate::errors::ConversionError>;
// }

/// A wrapper over the `google::type::money` protobuf provided by the `googleapis` ([link](https://github.com/googleapis/googleapis/blob/master/google/type/money.proto))
///
/// Note.- This is not a performant datatype if your application requires operating
/// with the inner amounts. If that's the case, use some other struct and convert
/// to this in a final step before serializing to the wire.
pub struct Money(crate::protos::google::r#type::Money);

impl Money {
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
}

// impl MoneyTrait for Money {
//     #[must_use]
//     fn amount(&self) -> BigDecimal {
//         self.as_ref().amount()
//     }

//     fn currency_code(&self) -> Result<CurrencyCode, crate::errors::ConversionError> {
//         self.as_ref().currency_code()
//     }
// }

impl ProtoWrapper<crate::protos::google::r#type::Money> for Money {
    // fn as_proto(&self) -> &crate::protos::google::r#type::Money {
    //     &self.0
    // }

    // fn as_ref<'a>(&'a self) -> Self::Reference<'a> {
    //     MoneyRef::from(&self.0)
    // }
}

impl From<Money> for crate::protos::google::r#type::Money {
    fn from(val: Money) -> Self {
        val.0
    }
}

impl From<crate::protos::google::r#type::Money> for Money {
    fn from(v: crate::protos::google::r#type::Money) -> Self {
        Self(v)
    }
}

// impl AsRef<crate::protos::google::r#type::Money> for Money {
//     fn as_ref(&self) -> &crate::protos::google::r#type::Money {
//         &self.0
//     }
// }

impl TryFrom<Vec<u8>> for Money {
    type Error = crate::errors::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::google::r#type::Money::decode(&*value)?))
    }
}

impl<'a> ProtoWrapperWithRef<'a, crate::protos::google::r#type::Money> for Money {
    type Reference = MoneyRef;

    fn as_ref(&'a self) -> &'a Self::Reference {
        MoneyRef::new(&self.0)
    }
}

impl Deref for Money {
    type Target = MoneyRef;

    fn deref(&self) -> &Self::Target {
        Self::Target::new(&self.0)
    }
}

// impl<'a> Deref for Money {
//     type Target = MoneyRef<'a>;

//     fn deref(&'a self) -> &'a Self::Target {
//         todo!()
//     }
// }

// impl<'a> AsRef<MoneyRef<'a>> for Money {
//     fn as_ref(&'a self) -> &MoneyRef<'a> {
//         &MoneyRef::from(&self.0)
//     }
// }

impl Mul<&BigDecimal> for &Money {
    type Output = Result<Money, crate::errors::ConversionError>;

    fn mul(self, rhs: &BigDecimal) -> Self::Output {
        let new_amount = self.amount() * rhs;
        Money::new(new_amount, self.currency_code()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn positive_amount() {
        let amount = BigDecimal::from_str("12.345600").unwrap();
        let money = Money::new(amount, CurrencyCode::EUR).unwrap();

        assert_eq!(money.0.currency_code, "EUR");
        assert_eq!(money.0.units, 12i64);
        assert_eq!(money.0.nanos, 345_600_000i32);

        assert_eq!(money.amount().to_string(), "12.3456");
        assert_eq!(money.currency_code().unwrap(), CurrencyCode::EUR);
    }

    #[test]
    fn negative_amount() {
        let amount = BigDecimal::from_str("-12345600").unwrap();
        let money = Money::new(amount, CurrencyCode::EUR).unwrap();

        assert_eq!(money.0.currency_code, "EUR");
        assert_eq!(money.0.units, -12345600i64);
        assert_eq!(money.0.nanos, 0i32);

        assert_eq!(money.amount().to_string(), "-12345600");
        assert_eq!(money.currency_code().unwrap(), CurrencyCode::EUR);
    }

    #[test]
    fn fractional_amount() {
        let amount = BigDecimal::from_str("-0.12345600").unwrap();
        let money = Money::new(amount, CurrencyCode::EUR).unwrap();

        assert_eq!(money.0.currency_code, "EUR");
        assert_eq!(money.0.units, -0i64);
        assert_eq!(money.0.nanos, -123_456_000i32);

        assert_eq!(money.amount().to_string(), "-0.123456");
        assert_eq!(money.currency_code().unwrap(), CurrencyCode::EUR);
    }

    #[test]
    fn zero_amount() {
        let amount = BigDecimal::from_str("0.00").unwrap();
        let money = Money::new(amount, CurrencyCode::USD).unwrap();

        assert_eq!(money.0.currency_code, "USD");
        assert_eq!(money.0.units, 0i64);
        assert_eq!(money.0.nanos, 0i32);

        assert_eq!(money.amount().to_string(), "0");
        assert_eq!(money.currency_code().unwrap(), CurrencyCode::USD);
    }

    #[test]
    fn multiply() {
        let amount = BigDecimal::from_str("2.00").unwrap();
        let money = Money::new(amount, CurrencyCode::USD).unwrap();

        let factor = BigDecimal::from_str("1.50").unwrap();

        let result = (&money * &factor).unwrap();
        assert_eq!(result.0.currency_code, "USD");
        assert_eq!(result.0.units, 3i64);
        assert_eq!(result.0.nanos, 0i32);
    }

    #[test]
    fn as_ref() {
        let amount = BigDecimal::from_str("2.00").unwrap();
        let money = Money::new(amount, CurrencyCode::USD).unwrap();

        let reference = money.as_ref();
        assert_eq!(reference.amount().to_string(), "2");
        assert_eq!(reference.currency_code().unwrap(), CurrencyCode::USD);
    }
}
