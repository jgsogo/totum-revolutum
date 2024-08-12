use std::fmt::{Display, Formatter};

use camino::Utf8PathBuf;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MONTHS_IN_ENGLISH: Vec<String> = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]
    .into_iter()
    .map(|s| s.to_lowercase())
    .collect();
    static ref MONTHS_IN_SPANISH: Vec<String> = vec![
        "Enero",
        "Febrero",
        "Marzo",
        "Abril",
        "Mayo",
        "Junio",
        "Julio",
        "Agosto",
        "Septiembre",
        "Octubre",
        "Noviembre",
        "Diciembre",
    ]
    .into_iter()
    .map(|s| s.to_lowercase())
    .collect();

    /// Regex to match YYYY/MM/DD or YYYY-MM-DD or YYYY_MM_DD or YYYYMMDD
    static ref RE_YYYY_MM_DD: Regex = Regex::new(r"(?<year>\d{4})[/\-_]?(?<month>\d{2})[/\-_]?(?<day>\d{2})").unwrap();

    /// Regex to match YYYY/<month>/DD or YYYY-<month>-DD or YYYY_<month>_DD
    static ref RE_YYYY_MONTH_DD: Regex = Regex::new(r"(?<year>\d{4})[/\-_](?<month>\w+)[/\-_](?<day>\d{2})").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Date {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
}

impl Date {
    pub fn new(year: Option<u16>, month: Option<u8>, day: Option<u8>) -> Self {
        Self { year, month, day }
    }
}

impl Display for Date {
    /// Returns a string with the format `YYYY/MM/DD` filled with the provided values. For those not
    /// available it will just return zeroes.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}/{:02}/{:02}",
            self.year.unwrap_or(0),
            self.month.unwrap_or(0),
            self.day.unwrap_or(0)
        )
    }
}

impl From<Date> for Utf8PathBuf {
    fn from(value: Date) -> Self {
        Utf8PathBuf::from(value.to_string())
    }
}

/// Tries to guess a date from the string using some regex expressions.
/// It returns a tuple of optionals with the year, month and day.
pub fn guess_date_from_str(value: &str) -> Option<Date> {
    // Numbers: Maybe the string contains the hint for a date
    let caps = if let Some(caps) = RE_YYYY_MM_DD.captures(value) {
        caps
    } else if let Some(caps) = RE_YYYY_MONTH_DD.captures(value) {
        caps
    } else {
        return None;
    };

    let year = check_candidate_year(&caps["year"]);
    let month = check_candidate_month(&caps["month"]);
    let day = check_candidate_day(&caps["day"]);
    if year.is_none() && month.is_none() && day.is_none() {
        None
    } else {
        Some(Date::new(year, month, day))
    }
}

/// A helper method to parse a year from a string. Expectation is just an u16 between 1900 and 2100
fn check_candidate_year(year: &str) -> Option<u16> {
    match year.parse::<u16>() {
        Ok(year) => {
            if !(1900..=2100).contains(&year) {
                None
            } else {
                Some(year)
            }
        }
        Err(_) => None,
    }
}

/// A helper method to parse a day from a string. Expectation is just an u8 in range [1, 31]
fn check_candidate_day(day: &str) -> Option<u8> {
    match day.parse::<u8>() {
        Ok(day) => {
            if day == 0 || day > 31 {
                None
            } else {
                Some(day)
            }
        }
        Err(_) => None,
    }
}

/// A helper method to parse a month from a string. It will match a number between 1 and 12, but
/// also month names in Spanish or English
fn check_candidate_month(month: &str) -> Option<u8> {
    //  * A number between 1 and 12
    match month.parse::<u8>() {
        Ok(month) => {
            if month == 0 || month > 12 {
                None
            } else {
                Some(month)
            }
        }
        Err(_) => {
            //  * A string with the month name (english, spanish)
            let months_lists: Vec<&Vec<String>> = vec![&*MONTHS_IN_ENGLISH, &*MONTHS_IN_SPANISH];

            months_lists.iter().find_map(|month_list| {
                month_list
                    .iter()
                    .position(|v| {
                        let input = month.to_lowercase();
                        v.starts_with(&input)
                    })
                    .map(|v| (v + 1) as u8)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_date_from_str() {
        // regex: basic values
        assert_eq!(guess_date_from_str("1984/01/01").unwrap().to_string(), "1984/01/01");
        assert_eq!(guess_date_from_str("1984-01-01").unwrap().to_string(), "1984/01/01");
        assert_eq!(guess_date_from_str("1984/01/01").unwrap().to_string(), "1984/01/01");
        assert_eq!(guess_date_from_str("1984-01_01").unwrap().to_string(), "1984/01/01");
        assert_eq!(guess_date_from_str("19840101").unwrap().to_string(), "1984/01/01");

        // regex: trailing data is skipped
        assert_eq!(guess_date_from_str("1984010122222").unwrap().to_string(), "1984/01/01");

        // regex: day overflow
        assert_eq!(guess_date_from_str("19840152").unwrap().to_string(), "1984/01/00");

        // regex: month overflow
        assert_eq!(guess_date_from_str("19841305").unwrap().to_string(), "1984/00/05");

        // regex: year overflow
        assert_eq!(guess_date_from_str("18700101").unwrap().to_string(), "0000/01/01");
        assert_eq!(guess_date_from_str("25000101").unwrap().to_string(), "0000/01/01");

        // testing months in english
        assert_eq!(guess_date_from_str("1984-jan-01").unwrap().to_string(), "1984/01/01");
        assert_eq!(
            guess_date_from_str("1984-october-01").unwrap().to_string(),
            "1984/10/01"
        );

        // testing months in spanish
        assert_eq!(guess_date_from_str("1984-abril-01").unwrap().to_string(), "1984/04/01");
        assert_eq!(guess_date_from_str("1984/dec/01").unwrap().to_string(), "1984/12/01");

        // no match
        assert_eq!(guess_date_from_str("1512-notamonth-50"), None);
    }

    #[test]
    fn test_as_utf8_path() {
        let date = guess_date_from_str("1984/01/01").unwrap();
        let as_utf8_date: Utf8PathBuf = date.into();
        assert_eq!(as_utf8_date.as_str(), "1984/01/01");
    }

    #[test]
    fn test_order() {
        let date1 = guess_date_from_str("1984/12/31").unwrap();
        let date2 = guess_date_from_str("1985/01/01").unwrap();
        assert!(date2 > date1);

        let date3 = guess_date_from_str("1985/02/01").unwrap();
        assert!(date3 > date2);

        let date4 = guess_date_from_str("1985/02/02").unwrap();
        assert!(date4 > date3);
    }
}
