use filesystem::FilesystemLocal;

pub type FilesystemLocalAsync = FilesystemLocal<async_std::fs::File>;
pub type FilesystemLocalSync = FilesystemLocal<std::fs::File>;

mod file;
mod filesystem;
mod parallel_visitor;
