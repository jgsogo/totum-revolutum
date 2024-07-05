use chrono::NaiveDateTime;
use exif::Exif;

use crate::metadata::collector::EXIF_KEY;
use crate::{CollectMetadataFrom, MetadataCollector};

impl CollectMetadataFrom<&Exif> for MetadataCollector {
    /// Collects some selected values from [`Exif`] object
    fn collect_from(&mut self, source: &Exif) -> serde_json::Value {
        let mut metadata = serde_json::json!({});

        // Date fields
        {
            let date_fields = vec![
                exif::Tag::DateTimeOriginal,
                exif::Tag::DateTime,
                exif::Tag::DateTimeDigitized,
            ];
            for it in date_fields {
                if let Some(field) = source.get_field(it, exif::In::PRIMARY) {
                    let new_value = field.display_value().with_unit(source);
                    let no_timezone = NaiveDateTime::parse_from_str(&new_value.to_string(), "%Y-%m-%d %H:%M:%S")
                        .expect("EXIF datetime format mismatch");
                    metadata[it.to_string()] = serde_json::json!(no_timezone.format("%Y/%m/%d").to_string());
                }
            }
        }

        // GPS fields
        {
            let candidate_files = vec![
                exif::Tag::GPSLatitude,
                exif::Tag::GPSLongitude,
                exif::Tag::GPSAltitude,
                exif::Tag::GPSTimeStamp,
            ];

            for it in candidate_files {
                if let Some(field) = source.get_field(it, exif::In::PRIMARY) {
                    let new_value = field.display_value().with_unit(source);
                    metadata[it.to_string()] = serde_json::json!(new_value.to_string());
                }
            }
        }

        if !metadata.as_object().unwrap().is_empty() {
            self.merge(EXIF_KEY, metadata.clone());
        }
        metadata
    }
}
