use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::protos::google::r#type::Decimal as DecimalProto;

use proto_wrapper::ProtoWrapper;

/// A wrapper over the `google::type::Decimal` protobuf provided by the `googleapis` ([link](https://github.com/googleapis/googleapis/blob/master/google/type/decimal.proto))
#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Decimal(DecimalProto);

impl Decimal {
    pub fn new(value: BigDecimal) -> Self {
        Self(DecimalProto {
            value: value.normalized().to_scientific_notation(),
        })
    }

    pub fn value(&self) -> Result<BigDecimal, crate::errors::ConversionError> {
        Ok(BigDecimal::from_str(&self.0.value)?)
    }
}

impl TryFrom<Decimal> for BigDecimal {
    type Error = crate::errors::ConversionError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        value.value()
    }
}

impl TryFrom<&Decimal> for BigDecimal {
    type Error = crate::errors::ConversionError;

    fn try_from(value: &Decimal) -> Result<Self, Self::Error> {
        value.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::Zero;

    #[test]
    fn roundtrip() {
        let amount = BigDecimal::from_str("-00.01234").unwrap();

        let dec = Decimal::new(amount);
        assert_eq!(dec.0.value, "-1.234e-2");

        let big_decimal: BigDecimal = dec.try_into().unwrap();
        assert_eq!(big_decimal.to_string(), "-0.01234");
    }

    #[test]
    fn roundtrip_zero() {
        let amount = BigDecimal::zero();

        let dec = Decimal::new(amount);
        assert_eq!(dec.0.value, "0e0");

        let big_decimal: BigDecimal = dec.try_into().unwrap();
        assert_eq!(big_decimal.to_string(), "0");
    }
}
