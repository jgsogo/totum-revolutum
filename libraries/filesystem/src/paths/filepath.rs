use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::path::Path;

use camino::{Utf8Path, Utf8PathBuf};

use crate::paths::directory_path::DirectoryPath;
use crate::paths::filename::Filename;

/// An owned, mutable valid UTF-8 path to a file (akin to [`String`]).
///
/// Note.- Implementation taken from [`Utf8PathBuf`].
#[derive(Debug, Eq, PartialEq, Clone)]
#[repr(transparent)]
pub struct FilePathBuf(Utf8PathBuf);

impl FilePathBuf {
    pub fn new(directory: impl AsRef<DirectoryPath>, filename: impl AsRef<Filename>) -> Self {
        let filepath = directory.as_ref().as_utf8_path().join(filename.as_ref().as_str());
        Self(filepath)
    }

    #[must_use]
    pub fn as_filepath(&self) -> &FilePath {
        // SAFETY: every FilePathBuf constructor ensures that self is a valid path
        unsafe { FilePath::assume_valid(&self.0) }
    }
}

impl Deref for FilePathBuf {
    type Target = FilePath;

    fn deref(&self) -> &FilePath {
        self.as_filepath()
    }
}

impl fmt::Display for FilePathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<FilePath> for FilePathBuf {
    fn as_ref(&self) -> &FilePath {
        self.as_filepath()
    }
}

impl AsRef<Utf8Path> for FilePathBuf {
    fn as_ref(&self) -> &Utf8Path {
        self.as_utf8_path()
    }
}

impl AsRef<Path> for FilePathBuf {
    fn as_ref(&self) -> &Path {
        self.as_std_path()
    }
}

impl AsRef<str> for FilePathBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A slice of a valid UTF8 path to a file (akin to str).
///
/// This type supports a number of operations for inspecting a path.
///
/// Note.- Implementation taken from [`Utf8Path`]
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct FilePath(Utf8Path);

impl FilePath {
    // invariant: Utf8Path must be guaranteed to be a valid path (it has been constructed using the [`validate_path`] function)
    #[inline]
    unsafe fn assume_valid(path: &Utf8Path) -> &FilePath {
        // SAFETY: FilePath is marked as #[repr(transparent)] so the conversion from a
        // *const Utf8Path to a *const FilePath is valid.
        &*(path as *const Utf8Path as *const FilePath)
    }

    pub fn directory(&self) -> &DirectoryPath {
        self.0
            .parent()
            .map(|v| unsafe { DirectoryPath::assume_valid(v) })
            .unwrap_or(DirectoryPath::root())
    }

    pub fn filename(&self) -> &Filename {
        unsafe { Filename::assume_valid(self.0.file_name().unwrap()) }
    }

    pub fn as_utf8_path(&self) -> &Utf8Path {
        &self.0
    }

    pub fn as_std_path(&self) -> &Path {
        self.0.as_std_path()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<FilePath> for FilePath {
    fn as_ref(&self) -> &FilePath {
        self
    }
}

impl AsRef<Utf8Path> for FilePath {
    fn as_ref(&self) -> &Utf8Path {
        self.as_utf8_path()
    }
}

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        self.as_std_path()
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{DirectoryPathBuf, FilenameBuf};

    use super::*;

    fn take_asref_file_path(value: impl AsRef<FilePath>, expected: &str) {
        let file_path: &FilePath = value.as_ref();
        assert_eq!(file_path.as_str(), expected);
    }

    fn take_asref_utf8_path(value: impl AsRef<Utf8Path>, expected: &str) {
        let file_path: &Utf8Path = value.as_ref();
        assert_eq!(file_path.as_str(), expected);
    }

    fn take_asref_std_path(value: impl AsRef<Path>, expected: &str) {
        let file_path: &Path = value.as_ref();
        assert_eq!(file_path.to_str().unwrap(), expected);
    }

    fn take_asref_str(value: impl AsRef<str>, expected: &str) {
        let file_path: &str = value.as_ref();
        assert_eq!(file_path, expected);
    }

    #[test]
    fn test_file_path_buf() {
        // check AsRef<FilePath> implementations
        let filepath_buf = FilePathBuf::new(
            DirectoryPathBuf::from_str("a/path").unwrap(),
            FilenameBuf::from_str("filename.txt").unwrap(),
        );
        take_asref_file_path(&filepath_buf, "a/path/filename.txt");
        take_asref_utf8_path(&filepath_buf, "a/path/filename.txt");
        take_asref_std_path(&filepath_buf, "a/path/filename.txt");
        take_asref_str(&filepath_buf, "a/path/filename.txt");
    }

    #[test]
    fn test_file_path() {
        let filepath_buf = FilePathBuf::new(
            DirectoryPathBuf::from_str("a/path").unwrap(),
            FilenameBuf::from_str("filename.txt").unwrap(),
        );
        let filepath: &FilePath = &filepath_buf;

        take_asref_file_path(filepath, "a/path/filename.txt");
        take_asref_utf8_path(filepath, "a/path/filename.txt");
        take_asref_std_path(filepath, "a/path/filename.txt");
        take_asref_str(filepath, "a/path/filename.txt");

        let dir = filepath.directory();
        assert_eq!(dir.as_str(), "a/path");
        let filename = filepath.filename();
        assert_eq!(filename.as_str(), "filename.txt");

        let filepath_in_root = FilePathBuf::new(DirectoryPath::root(), FilenameBuf::from_str("filename.txt").unwrap());
        let dir = filepath_in_root.directory();
        assert_eq!(dir.as_str(), "");
        let filename = filepath_in_root.filename();
        assert_eq!(filename.as_str(), "filename.txt");
    }
}
