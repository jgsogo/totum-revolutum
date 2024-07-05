mod collector;
mod from_exif;
mod from_filepath;
mod from_fs_metadata;

pub use collector::{parse_key_val, CollectMetadataFrom, MetadataCollector};
