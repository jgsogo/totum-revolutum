use crate::metadata::from_exif::DATE_FIELDS;

pub(crate) static CLI_CLAP_KEY: &str = "cli";
pub(crate) static EXIF_KEY: &str = "exif";
pub(crate) static FILEPATH_KEY: &str = "path";
pub(crate) static FS_METADATA_KEY: &str = "filesystem";

pub trait CollectMetadataFrom<T> {
    /// Collects metadata from the given `source` and stores it into `self`. Additionally, it returns
    /// all the metadata collected.
    ///
    /// Note that collected metadata will be **merged** using [`MetadataCollector::merge`].
    fn collect_from(&mut self, source: T) -> serde_json::Value;
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

    /// Inspects all the entries in the metadata, looking for the best candidate date. It applies
    /// several rules:
    ///  * Order of preference: CLI > EXIF > Filepath > ~fs::Metadata~
    pub fn get_candidate_date(&self) -> Option<&str> {
        if let Some(cli) = self.data.get(CLI_CLAP_KEY) {
            if let Some(date) = cli.get("date") {
                return date.as_str();
            }
        }
        if let Some(exif) = self.data.get(EXIF_KEY) {
            for it in &*DATE_FIELDS {
                if let Some(date) = exif.get(it.to_string()) {
                    return date.as_str();
                }
            }
        }
        if let Some(filepath) = self.data.get(FILEPATH_KEY) {
            if let Some(date) = filepath.get("date") {
                return date.as_str();
            }
        }
        // Created/Modified date can be very, very, very misleading...
        // else if let Some(fs_metadata) = self.data.get(FS_METADATA_KEY) {
        //     if let Some(date) = fs_metadata.get("created") {
        //         return Some(date.to_string());
        //     }
        // }

        None
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
    fn test_get_candidate_date() {
        // Test the order of preference of the different alternatives

        let mut collector = MetadataCollector::default();

        // No date if metadata is empty
        assert_eq!(collector.get_candidate_date(), None);

        // No date if `date` is not in the fields
        collector.add(
            CLI_CLAP_KEY,
            serde_json::json!({
               "other-thing": "other-thing-from-cli",
            }),
        );
        assert_eq!(collector.get_candidate_date(), None);

        // std::fs::Metadata is ignored
        collector.add(
            FS_METADATA_KEY,
            serde_json::json!({
               "created": "fs_metadata-created",
                "modified": "fs_metadata-modified",
            }),
        );
        assert_eq!(collector.get_candidate_date(), None);

        // filepath has the lowest precedence
        collector.add(
            FILEPATH_KEY,
            serde_json::json!({
               "date": "filepath-date",
            }),
        );
        assert_eq!(collector.get_candidate_date().unwrap(), "filepath-date");

        // exif data has higher precedence, check the order for its tags
        collector.merge(
            EXIF_KEY,
            serde_json::json!({"DateTimeDigitized": "exif-DateTimeDigitized"}),
        );
        assert_eq!(collector.get_candidate_date().unwrap(), "exif-DateTimeDigitized");
        collector.add(EXIF_KEY, serde_json::json!({"DateTime": "exif-DateTime"}));
        assert_eq!(collector.get_candidate_date().unwrap(), "exif-DateTime");
        collector.add(
            EXIF_KEY,
            serde_json::json!({"DateTimeOriginal": "exif-DateTimeOriginal"}),
        );
        assert_eq!(collector.get_candidate_date().unwrap(), "exif-DateTimeOriginal");

        // cli has even higher
        collector.add(
            CLI_CLAP_KEY,
            serde_json::json!({
               "date": "cli-date",
            }),
        );
        assert_eq!(collector.get_candidate_date().unwrap(), "cli-date");
    }
}
