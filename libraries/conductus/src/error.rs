use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum Error {
    #[error("All receivers are dropped")]
    AllReceiversAreDropped,

    #[error("Stop pipeline receiver: {0}")]
    StopIterator(String),
}
