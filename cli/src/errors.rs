use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIErrors {
    #[error("{0:?}")]
    ExitFailure(String),
}
