#[cfg(feature = "diesel")]
pub use diesel_indexed::FilesystemIndexedDB;
pub use filesystem_indexed::FilesystemIndexed;

#[cfg(feature = "diesel")]
mod diesel_indexed;
mod filesystem_indexed;
