use bigdecimal::BigDecimal;
use std::str::FromStr;

// FIXME: Move this to //libraries/googleapis and reuse it.

pub struct Decimal(crate::protos::google::r#type::Decimal);

impl Into<crate::protos::google::r#type::Decimal> for Decimal {
    fn into(self) -> crate::protos::google::r#type::Decimal {
        self.0
    }
}

impl From<BigDecimal> for Decimal {
    fn from(value: BigDecimal) -> Self {
        Self(crate::protos::google::r#type::Decimal {
            value: value.to_scientific_notation(),
        })
    }
}

impl TryInto<BigDecimal> for Decimal {
    type Error = bigdecimal::ParseBigDecimalError;
    fn try_into(self) -> Result<BigDecimal, Self::Error> {
        BigDecimal::from_str(&self.0.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::Zero;

    #[test]
    fn roundtrip() {
        let amount = BigDecimal::from_str("-00.01234").unwrap();

        let dec: Decimal = amount.into();
        assert_eq!(dec.0.value, "-1.234e-2");

        let big_decimal: BigDecimal = dec.try_into().unwrap();
        assert_eq!(big_decimal.to_string(), "-0.01234");
    }

    #[test]
    fn roundtrip_zero() {
        let amount = BigDecimal::zero();

        let dec: Decimal = amount.into();
        assert_eq!(dec.0.value, "0e0");

        let big_decimal: BigDecimal = dec.try_into().unwrap();
        assert_eq!(big_decimal.to_string(), "0");
    }
}
