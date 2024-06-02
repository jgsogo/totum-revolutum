use thiserror::Error;

/// An error that can be returned when creating a [`RemotePath`] from other type. Returned from
/// [`TryFrom`] implementations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("InvalidRemotePath {source}")]
pub struct InvalidRemotePathError {
    pub(crate) source: InvalidRemotePathKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidRemotePathKind {
    #[error("no absolute path")]
    NoAbsolutePath,

    #[error("resolved path is outside root folder")]
    OutsideRootFolder,

    #[error("not usable as a path to a file")]
    NotAFile,
}

/// An error that can be returned when creating a [`File`] from other type. Returned from
/// [`TryFrom`] implementations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("InvalidFile {source}")]
pub struct InvalidFileError {
    pub(crate) source: InvalidFileKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidFileKind {
    #[error(transparent)]
    InvalidRemotePath(#[from] InvalidRemotePathError),
}

/// An error that can be returned when creating a [`Folder`] from other type. Returned from
/// [`TryFrom`] implementations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("InvalidFolder {source}")]
pub struct InvalidFolderError {
    pub(crate) source: InvalidFolderKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidFolderKind {
    #[error(transparent)]
    InvalidRemotePath(#[from] InvalidRemotePathError),
}

/// An error that can be returned when parsing some element from a string. Returned from
/// [`std::str::FromStr`] trait implementations.
#[derive(Debug, Error)]
#[error("Cannot parse from string '{string}': {source}")]
pub struct ParseError {
    pub(crate) string: String,
    pub(crate) source: ParseErrorKind,
}

/// Additional information for [`ParseError`] error
#[derive(Debug, Error)]
pub enum ParseErrorKind {
    #[error("no FileID prefix, missing `fileid:`")]
    NoFileIDPrefix,

    #[error("no FolderID prefix, missing `folderid:`")]
    NoFolderIDPrefix,

    #[error("no RemotePath prefix, missing `path:`")]
    NoRemotePathPrefix,

    #[error("Not valid integer value")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    InvalidRemotePath(#[from] InvalidRemotePathError),

    #[error(transparent)]
    InvalidFile(#[from] InvalidFileError),

    #[error(transparent)]
    InvalidFolder(#[from] InvalidFolderError),

    #[error("Input can't be parsed as FileID or RemotePath")]
    InvalidFileIDOrRemotePath,

    #[error("Input can't be parsed as FolderID or RemotePath")]
    InvalidFolderIDOrRemotePath,
}
