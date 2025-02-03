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

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Error", 2)?;
        // state.serialize_field("severity", &self.severity())?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
