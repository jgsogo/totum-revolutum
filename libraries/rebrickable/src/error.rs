use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    SerializationError(#[from] DeserializationError),
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
