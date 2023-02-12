pub use file::File;
pub use file_metadata::FileMetadata;
pub use filesystem_t::Filesystem;

pub mod copy;
#[cfg(feature = "diff")]
pub mod diff;
mod file;
mod file_metadata;
mod filesystem_t;
#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "test_utils")]
pub mod mocks;
pub mod move_file;
mod utils;
