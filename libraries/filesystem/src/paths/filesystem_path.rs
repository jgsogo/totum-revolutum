use super::{DirectoryPathBuf, FilePathBuf};

/// Either a path to a directory ([`DirectoryPathBuf`]) or a path to a file ([`FilePathBuf`])
pub enum FilesystemPath {
    DirectoryPath(DirectoryPathBuf),
    FilePath(FilePathBuf),
}
