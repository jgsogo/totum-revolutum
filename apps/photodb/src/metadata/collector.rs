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
