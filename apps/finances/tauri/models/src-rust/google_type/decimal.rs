use crate::traits::ProtoWrapper;
use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::protos::google::r#type::Decimal as DecimalProto;
use crate::traits::private_parts::ProtoWrapperPrivate;

/// A wrapper over the `google::type::Decimal` protobuf provided by the `googleapis` ([link](https://github.com/googleapis/googleapis/blob/master/google/type/decimal.proto))
#[repr(transparent)]
pub struct Decimal(DecimalProto);

impl Decimal {
    pub fn new(value: BigDecimal) -> Self {
        Self(DecimalProto {
            value: value.to_scientific_notation(),
        })
    }
}

impl ProtoWrapperPrivate<DecimalProto> for Decimal {
    fn new_ref(proto: &DecimalProto) -> &Self {
        (unsafe { &*(proto as *const DecimalProto as *const Self) }) as _
    }

    fn inner_proto(&self) -> &DecimalProto {
        &self.0
    }
}

// impl ProtoWrapper<crate::protos::google::r#type::Money> for Money {}

// impl From<Decimal> for crate::protos::google::r#type::Decimal {
//     fn from(val: Decimal) -> Self {
//         val.0
//     }
// }

// impl From<crate::protos::google::r#type::Decimal> for Decimal {
//     fn from(v: crate::protos::google::r#type::Decimal) -> Self {
//         Self(v)
//     }
// }

// impl From<BigDecimal> for Decimal {
//     fn from(value: BigDecimal) -> Self {
//         Self(crate::protos::google::r#type::Decimal {
//             value: value.to_scientific_notation(),
//         })
//     }
// }

// impl TryFrom<Decimal> for BigDecimal {
//     type Error = crate::errors::ConversionError;

//     fn try_from(value: Decimal) -> Result<Self, Self::Error> {
//         Ok(BigDecimal::from_str(&value.0.value)?)
//     }
// }

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
