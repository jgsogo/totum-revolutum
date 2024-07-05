use std::time::UNIX_EPOCH;

use camino::Utf8Path;
use chrono::TimeZone;
use exif::Exif;
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

    /// Regex to match YYYY/MM/DD or YYYY-MM-DD or YYYY_MM_DD
    static ref RE_YYYY_MM_DD: Regex = Regex::new(r"(?<year>\d{4})[/-_](?<month>\d{2})[/-_](?<day>\d{2})").unwrap();
}

/// Collects metadata from different sources into a [`serde_json::Value`] object. It also offers
/// some functions and heuristics based on this metadata
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MetadataCollector {
    data: serde_json::Value,
}

impl Default for MetadataCollector {
    fn default() -> Self {
        Self {
            data: serde_json::json!({}),
        }
    }
}

impl AsRef<serde_json::Value> for MetadataCollector {
    fn as_ref(&self) -> &serde_json::Value {
        self.as_json_value()
    }
}

impl From<MetadataCollector> for serde_json::Value {
    fn from(value: MetadataCollector) -> Self {
        value.data
    }
}

impl MetadataCollector {
    /// Returns a reference to the inner [`serde_json::Value`]
    pub fn as_json_value(&self) -> &serde_json::Value {
        &self.data
    }

    /// Assign some `value` to the given `entry`. If `entry` already exists, it will be overridden
    pub fn add(&mut self, entry: &str, value: serde_json::Value) {
        self.data[entry] = value
    }

    /// Merges the existing object in `entry` with the given `value`. In case of conflict, values
    /// will be overridden by the ones coming from the input `value`.
    pub fn merge(&mut self, entry: &str, value: serde_json::Value) {
        let v = self.data.as_object_mut().unwrap();
        merge(v.entry(entry).or_insert(serde_json::Value::Null), value)
    }
}

/// Merges a [`serde_json::Value`] object into another. Existing values will be overridden by the
/// ones coming from `b`.
fn merge(lhs: &mut serde_json::Value, rhs: serde_json::Value) {
    match (lhs, rhs) {
        (lhs @ &mut serde_json::Value::Object(_), serde_json::Value::Object(rhs)) => {
            let lhs = lhs.as_object_mut().unwrap();
            for (k, v) in rhs {
                merge(lhs.entry(k).or_insert(serde_json::Value::Null), v);
            }
        }
        (lhs, rhs) => *lhs = rhs,
    }
}

/// Parse a single key-value pair (to be used for clap custom types)
pub fn parse_key_val(
    s: &str,
) -> Result<(String, serde_json::Value), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;

    let key: String = s[..pos].parse()?;

    let value = {
        let value: String = s[pos + 1..].parse()?;
        let (hint, value) = match value.find(':') {
            None => ("str", value.as_str()),
            Some(hint_found) => {
                let hint = &value[..hint_found];
                let value = &value[hint_found + 1..];
                (hint, value)
            }
        };
        match hint {
            "int" | "integer" | "i64" => {
                let value: i64 = value.parse()?;
                serde_json::json!(value)
            }
            "bool" | "boolean" => {
                let value: bool = value.parse()?;
                serde_json::json!(value)
            }
            "float" | "f64" => {
                let value: f64 = value.parse()?;
                serde_json::json!(value)
            }
            "list" | "array" => {
                // comma-separated list of strings
                let values: Vec<&str> = value.split(',').collect();
                serde_json::json!(values)
            }
            // Assume string
            _ => serde_json::json!(value),
        }
    };

    Ok((key, value))
}

pub trait CollectMetadataFrom<T> {
    /// Collects metadata from the given `source` and stores it into `self`. Additionally, it returns
    /// all the metadata collected.
    ///
    /// Note that collected metadata will be **merged** using [`MetadataCollector::merge`].
    fn collect_from(&mut self, source: T) -> serde_json::Value;
}

impl CollectMetadataFrom<&Exif> for MetadataCollector {
    /// Collects some selected values from [`Exif`] object
    fn collect_from(&mut self, source: &Exif) -> serde_json::Value {
        let candidate_files = vec![
            exif::Tag::DateTimeOriginal,
            exif::Tag::DateTime,
            exif::Tag::GPSLatitude,
            exif::Tag::GPSLongitude,
            exif::Tag::GPSAltitude,
            exif::Tag::GPSTimeStamp,
        ];

        let mut metadata = serde_json::json!({});

        // Store the fields as key-value pairs (using strings)
        for it in candidate_files {
            if let Some(field) = source.get_field(it, exif::In::PRIMARY) {
                let new_value = field.display_value().with_unit(source);
                metadata[it.to_string()] = serde_json::json!(new_value.to_string());
            }
        }

        if !metadata.as_object().unwrap().is_empty() {
            self.merge("exif", metadata.clone());
        }
        metadata
    }
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
            self.merge("path", metadata.clone());
        }
        metadata
    }
}

/// Tries to guess a date from the string using the regex in [`RE_YYYY_MM_DD`]. With the values
/// found it returns a string with the `YYYY/MM/DD` format where missing values are filled with
/// zeroes.
fn guess_date_from_str(value: &str) -> Option<String> {
    // Maybe the string contains the hint for a date
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
                    .map(|v| v as u8)
            })
        }
    }
}

impl CollectMetadataFrom<std::fs::Metadata> for MetadataCollector {
    /// Collects some metadata from [`std::fs::Metadata`]: creation and modification date
    fn collect_from(&mut self, source: std::fs::Metadata) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        if let Ok(created) = source.created() {
            if let Ok(duration) = created.duration_since(UNIX_EPOCH) {
                let t = chrono::Utc
                    .timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos())
                    .unwrap();
                metadata["created"] = serde_json::json!(t.to_string());
            }
        }
        if let Ok(modified) = source.modified() {
            if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                let t = chrono::Utc
                    .timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos())
                    .unwrap();
                metadata["modified"] = serde_json::json!(t.to_string());
            }
        }

        if !metadata.as_object().unwrap().is_empty() {
            self.merge("filesystem", metadata.clone());
        }
        metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_function() {
        let mut a = serde_json::json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities":[ "london", "paris" ],
            "was_string": "value",
            "was_single_value": "value",
        });

        let b = serde_json::json!({
            "title": "This is another title",
            "person" : {
                "firstName" : "Jane"
            },
            "cities":[ "colombo" ],
            "phone": 1234,
            "was_string": 2,
            "was_single_value": {
                "a": 42,
            },
        });

        merge(&mut a, b);

        assert_eq!(a["title"].as_str().unwrap(), "This is another title");
        let person = a["person"].as_object().unwrap();
        assert_eq!(person["firstName"].as_str().unwrap(), "Jane");
        assert_eq!(person["lastName"].as_str().unwrap(), "Doe");
        let cities = a["cities"].as_array().unwrap();
        assert_eq!(cities.len(), 1);
        assert_eq!(cities.get(0).unwrap(), "colombo");
        assert_eq!(a["phone"].as_i64().unwrap(), 1234);
        assert_eq!(a["was_string"].as_i64().unwrap(), 2);
        assert!(a["was_single_value"].is_object());
        let was_single_value = a["was_single_value"].as_object().unwrap();
        assert_eq!(was_single_value["a"].as_i64().unwrap(), 42);
    }

    #[test]
    fn test_collector_default() {
        let collector = MetadataCollector::default();
        assert_eq!(collector.as_json_value(), &serde_json::json!({}));
    }

    #[test]
    fn test_collector_add() {
        let mut collector = MetadataCollector::default();

        // Add new field
        {
            let a = serde_json::json!({
                "title": "This is a title",
                "person" : {
                    "firstName" : "John",
                },
            });
            collector.add("a", a.clone());

            let collected = collector.as_json_value();
            assert_eq!(collected["a"], a);
        }

        // Add existing field (override everything)
        {
            let a = serde_json::json!({
                "just": "override",
            });
            collector.add("a", a.clone());

            let collected = collector.as_json_value();
            assert_eq!(collected["a"], a);
        }
    }

    #[test]
    fn test_collector_merge() {
        let mut collector = MetadataCollector::default();

        // Insert new field
        {
            let a = serde_json::json!({
                "title": "This is a title",
                "person" : {
                    "firstName" : "John",
                },
                "phone": 1234,
            });
            collector.merge("a", a.clone());

            let collected = collector.as_json_value();
            assert_eq!(collected["a"], a);
        }

        // Merge on top of existing data
        {
            let mut a = serde_json::json!({
                "title": "This is another title",
                "person" : {
                    "firstName" : "Jane",
                    "lastName" : "Doe"
                },
            });
            collector.merge("a", a.clone());

            let collected = collector.as_json_value();
            a["phone"] = serde_json::json!(1234);
            assert_eq!(collected["a"], a);
        }
    }

    #[test]
    fn test_parse_key_value() {
        {
            let (key, value) = parse_key_val("key=value").unwrap();
            assert_eq!(key, "key");
            assert_eq!(value, serde_json::json!("value"));
        }
        {
            assert_eq!(parse_key_val("key=int:123").unwrap().1, serde_json::json!(123));
            assert_eq!(parse_key_val("key=integer:123").unwrap().1, serde_json::json!(123));
            assert_eq!(parse_key_val("key=i64:-123").unwrap().1, serde_json::json!(-123));
        }
        {
            assert_eq!(parse_key_val("key=bool:true").unwrap().1, serde_json::json!(true));
            assert_eq!(parse_key_val("key=boolean:true").unwrap().1, serde_json::json!(true));
            assert_eq!(parse_key_val("key=bool:false").unwrap().1, serde_json::json!(false));
            assert_eq!(parse_key_val("key=boolean:false").unwrap().1, serde_json::json!(false));
        }
        {
            assert_eq!(parse_key_val("key=float:0.23").unwrap().1, serde_json::json!(0.23f64));
            assert_eq!(parse_key_val("key=f64:-0.23").unwrap().1, serde_json::json!(-0.23f64));
            assert_eq!(parse_key_val("key=float:1.e6").unwrap().1, serde_json::json!(1e6f64));
        }
        {
            assert_eq!(
                parse_key_val("key=list:1,2,3").unwrap().1,
                serde_json::json!(vec!["1", "2", "3"])
            );
            assert_eq!(
                parse_key_val("key=array:-1,-2,-3").unwrap().1,
                serde_json::json!(vec!["-1", "-2", "-3"])
            );
            assert_eq!(
                parse_key_val("key=list:tag1,tag2,tag 3").unwrap().1,
                serde_json::json!(vec!["tag1", "tag2", "tag 3"])
            );
        }
    }
}
