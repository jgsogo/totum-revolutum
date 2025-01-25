use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),

    #[error(transparent)]
    ModelProtoError(#[from] finances_app_models::Error),

    #[error(transparent)]
    ModelProtoConversionError(#[from] finances_app_models::errors::ConversionError),

    #[error("{0}")]
    Other(String),
}
