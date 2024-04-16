pub use r#trait::{Params, ParamsType};
pub use source_and_target_file::SourceAndTargetFile;
pub use source_and_target_folder::SourceAndTargetFolder;
pub use target_location::TargetLocation;

mod file;
mod folder;
mod source_and_target_file;
mod source_and_target_folder;
mod target_location;
mod r#trait;
