use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Serialization error '{error}': {content:?}")]
    SerializationError { error: serde_json::Error, content: String },
}
