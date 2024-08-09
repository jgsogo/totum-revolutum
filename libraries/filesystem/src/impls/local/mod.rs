use filesystem::FilesystemLocal;

/// Implementation of [`Filesystem`] using async [`async_std::fs::File`] files
pub type FilesystemLocalAsync = FilesystemLocal<async_std::fs::File>;
/// Implementation of [`Filesystem`] using regular [`std::fs::File`] files.
pub type FilesystemLocalSync = FilesystemLocal<std::fs::File>;

mod file;
mod filesystem;
mod parallel_visitor;
