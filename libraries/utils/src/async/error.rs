use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Arguments were already provided")]
    DuplicatedArguments,

    #[error("No arguments were provided")]
    MissingArguments,

    #[error("Task is already running")]
    TaskIsAlreadyRunning,
}
