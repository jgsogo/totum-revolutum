use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    DeserializationError(#[from] DeserializationError),

    #[error(transparent)]
    SerializationError(#[from] SerializationError),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    ParseError(#[from] crate::types::errors::ParseError),

    #[error("The file already exists")]
    FileAlreadyExists,

    #[error(transparent)]
    PCloudError(#[from] PCloudError),

    #[error("Wrong input data: {0}")]
    InputDataError(String),
}

/// An error that can be returned when serializing data.
#[derive(Debug, Error)]
#[error("Cannot deserialize string '{string}': {source}")]
pub struct SerializationError {
    pub string: String,
    pub source: SerializationErrorKind,
}

/// Additional information for [`SerializationError`] error
#[derive(Debug, Error)]
pub enum SerializationErrorKind {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
}

/// An error that can be returned when deserializing data.
#[derive(Debug, Error)]
#[error("Cannot deserialize string '{string}': {source}")]
pub struct DeserializationError {
    pub string: String,
    pub source: DeserializationErrorKind,
}

/// Additional information for [`DeserializationError`] error
#[derive(Debug, Error)]
pub enum DeserializationErrorKind {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error("Data field is empty")]
    EmptyDataField,
}

impl From<utils::http::Error> for Error {
    fn from(value: utils::http::Error) -> Self {
        match value {
            utils::http::Error::ReqwestError(r) => r.into(),
            utils::http::Error::DeserializationError(r) => {
                let source = match r.source {
                    utils::http::error::DeserializationErrorKind::SerdeError(r) => {
                        DeserializationErrorKind::SerdeError(r)
                    }
                };
                DeserializationError {
                    string: r.string,
                    source,
                }
                .into()
            }
        }
    }
}

#[derive(Debug, Error)]
#[error("Cannot deserialize string '{string}': {source}")]
pub enum PCloudError {
    #[error("(2005) Directory does not exist")]
    DirectoryDoesNotExist,

    #[error("{code}: {message}")]
    UnclassifiedError { code: u16, message: String },
}

impl From<(u16, String)> for Error {
    fn from(value: (u16, String)) -> Self {
        let (code, message) = value;
        let err = match code {
            2005 => PCloudError::DirectoryDoesNotExist,
            _ => PCloudError::UnclassifiedError { code, message },
        };
        err.into()
    }
}
