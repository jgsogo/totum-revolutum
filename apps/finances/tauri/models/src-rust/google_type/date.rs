use chrono::{Datelike, NaiveDate};

use crate::protos::google::r#type::Date as DateProto;

use proto_wrapper::ProtoWrapper;
use proto_wrapper_derive::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Date(DateProto);

impl Date {
    /// Creates a new [`Date`] following the same rules as the [`NaiveDate::from_ymd_opt`] implementation
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, crate::errors::ConversionError> {
        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::errors::ConversionError::FromDateComponents {
                year,
                month: month.try_into()?,
                day: day.try_into()?,
            })?;
        Ok(date.into())
    }
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Self(crate::protos::google::r#type::Date {
            year: value.year(),
            month: value.month() as i32,
            day: value.day() as i32,
        })
    }
}

impl TryFrom<Date> for NaiveDate {
    type Error = crate::errors::ConversionError;

    fn try_from(val: Date) -> Result<Self, Self::Error> {
        NaiveDate::from_ymd_opt(val.0.year, val.0.month as u32, val.0.day as u32).ok_or(
            crate::errors::ConversionError::FromDateComponents {
                year: val.0.year,
                month: val.0.month,
                day: val.0.day,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 10).unwrap();

        let date_proto: Date = date.into();
        assert_eq!(date_proto.0.year, 2025);
        assert_eq!(date_proto.0.month, 1);
        assert_eq!(date_proto.0.day, 10);

        let naive_date: NaiveDate = date_proto.try_into().unwrap();
        assert_eq!(naive_date.to_string(), "2025-01-10");
    }
}
