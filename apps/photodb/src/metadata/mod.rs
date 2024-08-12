pub use collector::{CollectMetadataFrom, MetadataCollector};
pub use from_cli_clap::MetadataCLI;

mod collector;
mod from_cli_clap;
mod from_exif;
mod from_filepath;
mod from_fs_metadata;
