use super::FileID;

/// A file in pCloud is represented by either a String/path or a FileID
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PCloudFile {
    FileID(FileID),
    Path(String),
}
