use crate::{CollectMetadataFrom, MetadataCollector};
use exif::Exif;

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
