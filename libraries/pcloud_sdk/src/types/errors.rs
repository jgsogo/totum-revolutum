use thiserror::Error;

/// An error that can be returned when parsing some element from a string. Returned from [`FromStr`]
/// trait implementations.
#[derive(Debug, Error)]
#[error("Cannot parse from string '{string}': {source}")]
pub struct ParseError {
    pub string: String,
    pub source: ParseErrorKind,
}

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
    Other(#[from] anyhow::Error),
}

// #[derive(Debug, Error)]
// pub enum TypeError {
//
//     ParseFileIDError(#[from] ParseError),
//
// }
