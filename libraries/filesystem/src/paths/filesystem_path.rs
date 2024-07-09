use super::{DirectoryPathBuf, FilePathBuf};

/// Either a path to a directory ([`DirectoryPathBuf`]) or a path to a file ([`FilePathBuf`])
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FilesystemPath {
    DirectoryPath(DirectoryPathBuf),
    FilePath(FilePathBuf),
}
