use camino::Utf8Path;

use crate::metadata::collector::FILEPATH_KEY;
use crate::{CollectMetadataFrom, MetadataCollector};

impl CollectMetadataFrom<&Utf8Path> for MetadataCollector {
    /// Collects metadata from a filesystem path: try to guess the date from directory and/or
    /// filesystem name.
    fn collect_from(&mut self, source: &Utf8Path) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        if let Some(date) = utils::dates::guess_date_from_str(source.as_str()) {
            metadata["date"] = serde_json::json!(date.to_string());
        }

        if !metadata.as_object().unwrap().is_empty() {
            self.merge(FILEPATH_KEY, metadata.clone());
        }
        metadata
    }
}
