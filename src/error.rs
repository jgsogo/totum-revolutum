// error1.rs

use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    APIError(#[from] reqwest::Error),
    SerializationError(#[from] serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "lsl")
    }
}
