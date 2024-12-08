use crate::models::ErrorContent;
use thiserror::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    DeserializationError(#[from] DeserializationError),

    #[error(transparent)]
    OXRError(ErrorContent),
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
}
