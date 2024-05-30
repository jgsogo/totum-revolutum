use thiserror::Error;

/// An error that can be returned when parsing some element from a string.
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

    #[error("Not valid integer value")]
    ParseInt(#[from] std::num::ParseIntError),
}

// #[derive(Debug, Error)]
// pub enum TypeError {
//
//     ParseFileIDError(#[from] ParseError),
//
// }
