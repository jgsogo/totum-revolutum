use clap::Args;

use utils::dates;

use crate::metadata::collector::CLI_CLAP_KEY;
use crate::{CollectMetadataFrom, MetadataCollector};

#[derive(Args, Debug)]
pub struct MetadataCLI {
    /// User provided metadata
    #[arg(short='m', long, value_parser = parse_key_val)]
    metadata: Vec<(String, serde_json::Value)>,
}

impl CollectMetadataFrom<&MetadataCLI> for MetadataCollector {
    fn collect_from(&mut self, source: &MetadataCLI) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        for (key, value) in &source.metadata {
            metadata[key] = value.clone();
        }

        if !metadata.as_object().unwrap().is_empty() {
            self.merge(CLI_CLAP_KEY, metadata.clone());
        }
        metadata
    }
}

/// Parse a single key-value pair (to be used for clap custom types)
fn parse_key_val(s: &str) -> Result<(String, serde_json::Value), Box<dyn std::error::Error + Send + Sync + 'static>> {
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
            "date" => {
                let date = dates::guess_date_from_str(value)
                    .ok_or_else(|| format!("String '{value}' cannot be parsed as date. Use YYYY/MM/DD"))?;
                serde_json::json!(date.to_string())
            }
            // Assume string
            _ => serde_json::json!(value),
        }
    };

    Ok((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        {
            assert_eq!(
                parse_key_val("key=date:1984/01/01").unwrap().1,
                serde_json::json!("1984/01/01")
            );
            assert_eq!(
                parse_key_val("key=date:1984/diciembre/01").unwrap().1,
                serde_json::json!("1984/12/01")
            );
            assert_eq!(
                parse_key_val("key=date:1984/12/52").unwrap().1,
                serde_json::json!("1984/12/00")
            );
        }
    }
}
