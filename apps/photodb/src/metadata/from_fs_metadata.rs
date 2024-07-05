use crate::{CollectMetadataFrom, MetadataCollector};
use chrono::TimeZone;
use std::time::UNIX_EPOCH;

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
