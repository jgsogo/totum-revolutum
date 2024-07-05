use crate::metadata::collector::FILEPATH_KEY;
use crate::{CollectMetadataFrom, MetadataCollector};
use camino::Utf8Path;
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

impl CollectMetadataFrom<&Utf8Path> for MetadataCollector {
    /// Collects metadata from a filesystem path: try to guess the date from directory and/or
    /// filesystem name.
    fn collect_from(&mut self, source: &Utf8Path) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        if let Some(guess) = guess_date_from_str(source.as_str()) {
            metadata["date"] = serde_json::json!(guess);
        }

        if !metadata.as_object().unwrap().is_empty() {
            self.merge(FILEPATH_KEY, metadata.clone());
        }
        metadata
    }
}

/// Tries to guess a date from the string using the regex in [`RE_YYYY_MM_DD`]. With the values
/// found it returns a string with the `YYYY/MM/DD` format where missing values are filled with
/// zeroes.
fn guess_date_from_str(value: &str) -> Option<String> {
    // Numbers: Maybe the string contains the hint for a date
    if let Some(caps) = RE_YYYY_MM_DD.captures(value) {
        let year = check_candidate_year(&caps["year"]);
        let month = check_candidate_month(&caps["month"]);
        let day = check_candidate_day(&caps["day"]);
        if year.is_some() || month.is_some() || day.is_some() {
            Some(format!(
                "{:04}/{:02}/{:02}",
                year.unwrap_or(0),
                month.unwrap_or(0),
                day.unwrap_or(0)
            ))
        } else {
            None
        }
    } else if let Some(caps) = RE_YYYY_MONTH_DD.captures(value) {
        let year = check_candidate_year(&caps["year"]);
        let month = check_candidate_month(&caps["month"]);
        let day = check_candidate_day(&caps["day"]);
        if year.is_some() || month.is_some() || day.is_some() {
            Some(format!(
                "{:04}/{:02}/{:02}",
                year.unwrap_or(0),
                month.unwrap_or(0),
                day.unwrap_or(0)
            ))
        } else {
            None
        }
    } else {
        None
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
    use crate::metadata::from_filepath::guess_date_from_str;

    #[test]
    fn test_guess_date_from_str() {
        // regex: basic values
        assert_eq!(&guess_date_from_str("1984/01/01").unwrap(), "1984/01/01");
        assert_eq!(&guess_date_from_str("1984-01-01").unwrap(), "1984/01/01");
        assert_eq!(&guess_date_from_str("1984/01/01").unwrap(), "1984/01/01");
        assert_eq!(&guess_date_from_str("1984-01_01").unwrap(), "1984/01/01");
        assert_eq!(&guess_date_from_str("19840101").unwrap(), "1984/01/01");

        // regex: trailing data is skipped
        assert_eq!(&guess_date_from_str("1984010122222").unwrap(), "1984/01/01");

        // regex: day overflow
        assert_eq!(&guess_date_from_str("19840152").unwrap(), "1984/01/00");

        // regex: month overflow
        assert_eq!(&guess_date_from_str("19841305").unwrap(), "1984/00/05");

        // regex: year overflow
        assert_eq!(&guess_date_from_str("18700101").unwrap(), "0000/01/01");
        assert_eq!(&guess_date_from_str("25000101").unwrap(), "0000/01/01");

        // testing months in english
        assert_eq!(&guess_date_from_str("1984-jan-01").unwrap(), "1984/01/01");
        assert_eq!(&guess_date_from_str("1984-october-01").unwrap(), "1984/10/01");

        // testing months in spanish
        assert_eq!(&guess_date_from_str("1984-abril-01").unwrap(), "1984/04/01");
        assert_eq!(&guess_date_from_str("1984/dec/01").unwrap(), "1984/12/01");
    }
}
