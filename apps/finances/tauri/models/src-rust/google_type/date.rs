use chrono::{Datelike, NaiveDate};

// FIXME: Move this to //libraries/googleapis and reuse it.

pub struct Date(crate::protos::google::r#type::Date);

impl Into<crate::protos::google::r#type::Date> for Date {
    fn into(self) -> crate::protos::google::r#type::Date {
        self.0
    }
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Self(crate::protos::google::r#type::Date {
            year: value.year() as i32,
            month: value.month() as i32,
            day: value.day() as i32,
        })
    }
}

impl Into<NaiveDate> for Date {
    fn into(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.0.year, self.0.month as u32, self.0.day as u32).unwrap()
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

        let naive_date: NaiveDate = date_proto.into();
        assert_eq!(naive_date.to_string(), "2025-01-10");
    }
}
