use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Path is outside filesystem")]
    PathOutsideFilesystem,

    #[error("Path refers to the root itself")]
    PathIsRoot,

    #[error("Path doesn't exist")]
    PathDoesNotExist,

    #[error("Target file already exists")]
    TargetFileExists,

    #[error("Source file doesn't exist")]
    SourceFileDoesNotExist,

    #[error("Given path is not a path to a file")]
    NotAFilepath,

    #[error("Operation forbidden")]
    Forbidden,

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("{0}")]
    Other(String),
}
