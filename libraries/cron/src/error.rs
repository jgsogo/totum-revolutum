use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] cron_parser::ParseError),

    #[error("{0}")]
    Other(String),
}
