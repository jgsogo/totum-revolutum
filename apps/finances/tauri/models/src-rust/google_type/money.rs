use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;

// FIXME: Move this to //libraries/googleapis and reuse it.

const NANO_EXP: usize = 9;
const NANO_VALUE: u32 = 1_000_000_000;

pub struct Money(pub(crate) crate::protos::google::r#type::Money);

impl From<Money> for crate::protos::google::r#type::Money {
    fn from(val: Money) -> Self {
        val.0
    }
}

impl From<(BigDecimal, String)> for Money {
    fn from(value: (BigDecimal, String)) -> Self {
        let (amount, ccy) = value;

        if amount.is_zero() {
            return Self(crate::protos::google::r#type::Money {
                currency_code: ccy,
                units: i64::zero(),
                nanos: i32::zero(),
            });
        }

        let (int_val, scale) = amount.as_bigint_and_scale();
        let (sign, mut digits) = int_val.to_radix_be(10);
        let (integer_part, fractional_part) = if scale <= 0 {
            let scale: usize = scale
                .unsigned_abs()
                .try_into()
                .expect("BigDecimal scale expected to fit into usize");
            digits.extend(std::iter::repeat(0).take(scale));
            let integer_part = num_bigint::BigInt::from_radix_be(sign, &digits, 10).expect("Radix_be no reversible");
            (integer_part, num_bigint::BigInt::zero())
        } else {
            let scale: usize = scale.try_into().expect("BigDecimal scale expected to fit into usize");
            let idx = digits.len() - scale;
            let integer_part =
                num_bigint::BigInt::from_radix_be(sign, &digits[0..idx], 10).expect("Radix_be no reversible");
            let mut fractional_digits = digits[idx..].to_vec();
            fractional_digits.resize(NANO_EXP, u8::zero());
            let fractional_part =
                num_bigint::BigInt::from_radix_be(sign, &fractional_digits, 10).expect("Radix_be no reversible");
            (integer_part, fractional_part)
        };

        Self(crate::protos::google::r#type::Money {
            currency_code: ccy,
            units: integer_part
                .to_i64()
                .expect("Integer part of input money doesn't fit into i64"),
            nanos: fractional_part
                .to_i32()
                .expect("Fractional part of input money doesn't fit into i32"),
        })
    }
}

impl From<Money> for (BigDecimal, String) {
    fn from(val: Money) -> Self {
        let ccy = val.0.currency_code;
        let amount = BigDecimal::from(val.0.units);
        let fractional = BigDecimal::from(val.0.nanos) / NANO_VALUE;
        (amount + fractional, ccy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn positive_amount() {
        let ccy = "EUR".to_string();
        let amount = BigDecimal::from_str("12.345600").unwrap();
        let money: Money = (amount, ccy).into();

        assert_eq!(money.0.currency_code, "EUR");
        assert_eq!(money.0.units, 12i64);
        assert_eq!(money.0.nanos, 345_600_000i32);

        let (amount, ccy): (BigDecimal, String) = money.into();
        assert_eq!(amount.to_string(), "12.3456");
        assert_eq!(ccy, "EUR");
    }

    #[test]
    fn negative_amount() {
        let ccy = "EUR".to_string();
        let amount = BigDecimal::from_str("-12345600").unwrap();
        let money: Money = (amount, ccy).into();

        assert_eq!(money.0.currency_code, "EUR");
        assert_eq!(money.0.units, -12345600i64);
        assert_eq!(money.0.nanos, 0i32);

        let (amount, ccy): (BigDecimal, String) = money.into();
        assert_eq!(amount.to_string(), "-12345600");
        assert_eq!(ccy, "EUR");
    }

    #[test]
    fn fractional_amount() {
        let ccy = "EUR".to_string();
        let amount = BigDecimal::from_str("-0.12345600").unwrap();
        let money: Money = (amount, ccy).into();

        assert_eq!(money.0.currency_code, "EUR");
        assert_eq!(money.0.units, -0i64);
        assert_eq!(money.0.nanos, -123_456_000i32);

        let (amount, ccy): (BigDecimal, String) = money.into();
        assert_eq!(amount.to_string(), "-0.123456");
        assert_eq!(ccy, "EUR");
    }

    #[test]
    fn zero_amount() {
        let ccy = "MKD".to_string();
        let amount = BigDecimal::from_str("0.00").unwrap();
        let money: Money = (amount, ccy).into();

        assert_eq!(money.0.currency_code, "MKD");
        assert_eq!(money.0.units, 0i64);
        assert_eq!(money.0.nanos, 0i32);

        let (amount, ccy): (BigDecimal, String) = money.into();
        assert_eq!(amount.to_string(), "0");
        assert_eq!(ccy, "MKD");
    }
}
