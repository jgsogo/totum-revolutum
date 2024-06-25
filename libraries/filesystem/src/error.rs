use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Path is outside filesystem")]
    PathOutsideFilesystem,

    #[error("Filename contains invalid characters")]
    InvalidFilename,

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

    #[error("Directory is not empty")]
    NotEmptyDirectory,

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[cfg(feature = "diesel_indexed_impl")]
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error("{0}")]
    Other(String),
}

#[cfg(feature = "diesel_indexed_impl")]
impl From<diesel_utils::error::Error> for Error {
    fn from(value: diesel_utils::error::Error) -> Self {
        match value {
            diesel_utils::error::Error::ObjectDoesNotExist(_) => Error::PathDoesNotExist,
            diesel_utils::error::Error::OtherDieselError(e) => e.into(),
        }
    }
}
