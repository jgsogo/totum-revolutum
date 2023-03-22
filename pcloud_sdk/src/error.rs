use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Serialization error '{error}': {content:?}")]
    SerializationError { error: serde_json::Error, content: String },

    #[error("API error {code}: {message}")]
    ApiError { code: u16, message: String },

    #[error("Cannot parse '{string}' to folder. Provide a 'folderid:<id>' or absolute path")]
    ParseFolderError { string: String },

    #[error("Cannot parse '{string}' to FolderID. Use format 'folderid:<id>'.")]
    ParseFolderIDError { string: String },

    #[error("Cannot parse '{string}' to RemotePath. Use format 'path:/absolute/path'.")]
    ParseRemotePathError { string: String },

    #[error("Cannot parse '{string}' to file. Provide a 'fileid:<id>' or absolute path")]
    ParseFileError { string: String },

    #[error("Cannot parse '{string}' to FileID. Use format 'fileid:<id>'.")]
    ParseFileIDError { string: String },
}
