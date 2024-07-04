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

pub trait PhotoMetadata {
    /// Collect metadata from this object into the given [`Value`]
    fn collect_metadata(&self) -> serde_json::Value;
}

impl PhotoMetadata for Exif {
    /// Collect from [`Exif`] **only** the metadata that will be stored in the DB
    fn collect_metadata(&self) -> serde_json::Value {
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
            if let Some(field) = self.get_field(it, exif::In::PRIMARY) {
                let new_value = field.display_value().with_unit(self);
                metadata[it.to_string()] = serde_json::json!(new_value.to_string());
            }
        }

        metadata
    }
}

impl PhotoMetadata for &Utf8Path {
    /// Guess some metadata from a path (it assumes it is a path to a file)
    fn collect_metadata(&self) -> serde_json::Value {
        let mut json_metadata = serde_json::json!({});

        if let Some(directory) = self.parent() {
            json_metadata["directory"] = directory.as_str().collect_metadata();
            json_metadata["directory"]["original"] = serde_json::json!(directory);
        }
        if let Some(filename) = self.file_name() {
            json_metadata["filename"] = filename.collect_metadata();
            json_metadata["filename"]["original"] = serde_json::json!(filename);
        }

        // Now from the filesystem itself
        if let Ok(metadata) = std::fs::metadata(self) {
            if let Ok(created) = metadata.created() {
                if let Ok(duration) = created.duration_since(UNIX_EPOCH) {
                    let t = chrono::Utc
                        .timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos())
                        .unwrap();
                    json_metadata["filesystem_created"] = serde_json::json!(t.to_string());
                }
            }
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                    let t = chrono::Utc
                        .timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos())
                        .unwrap();
                    json_metadata["filesystem_modified"] = serde_json::json!(t.to_string());
                }
            }
        }

        json_metadata
    }
}

impl PhotoMetadata for &str {
    /// Matches the string against [`crate::metadata::RE_YYYY_MM_DD`] and populates `date`
    fn collect_metadata(&self) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        // Maybe the string contains the hint for a date
        if let Some(caps) = RE_YYYY_MM_DD.captures(self) {
            let year = check_candidate_year(&caps["year"]);
            let month = check_candidate_month(&caps["month"]);
            let day = check_candidate_day(&caps["day"]);
            if year.is_some() || month.is_some() || day.is_some() {
                metadata["date"] = serde_json::json!(format!(
                    "{:04}/{:02}/{:02}",
                    year.unwrap_or(0),
                    month.unwrap_or(0),
                    day.unwrap_or(0)
                ));
            }
        };

        metadata
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
