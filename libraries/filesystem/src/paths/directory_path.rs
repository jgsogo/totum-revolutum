use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;

use camino::{Utf8Path, Utf8PathBuf};

use utils::filesystem::normalize_path;

use crate::{Error, Result};

const ROOT_DIR: &str = "";

fn validate_path(path: &Utf8Path) -> Result<Utf8PathBuf> {
    let path: Utf8PathBuf = normalize_path(path);
    if path.starts_with("../") {
        Err(Error::PathOutsideFilesystem)
    } else if path.is_absolute() {
        Err(Error::PathIsRoot)
    } else {
        // TODO: Do we want to validate that all characters are "valid"? What means "valid" here?
        Ok(path)
    }
}

/// An owned, mutable valid UTF-8 path to a directory (akin to [`String`]).
///
/// Note.- Implementation taken from [`Utf8PathBuf`].
#[derive(Debug, Eq, PartialEq, Clone)]
#[repr(transparent)]
pub struct DirectoryPathBuf(Utf8PathBuf);

impl DirectoryPathBuf {
    pub fn root() -> DirectoryPathBuf {
        let root = Utf8PathBuf::from(ROOT_DIR);
        DirectoryPathBuf(root)
    }

    #[must_use]
    pub fn as_directory_path(&self) -> &DirectoryPath {
        // SAFETY: every Utf8PathBuf constructor ensures that self is valid UTF-8
        unsafe { DirectoryPath::assume_valid(&self.0) }
    }
}

impl FromStr for DirectoryPathBuf {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let utf8_path = Utf8PathBuf::from(s);
        let utf8_path = validate_path(&utf8_path)?;
        Ok(Self(utf8_path))
    }
}

impl Deref for DirectoryPathBuf {
    type Target = DirectoryPath;

    fn deref(&self) -> &DirectoryPath {
        self.as_directory_path()
    }
}

impl fmt::Display for DirectoryPathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<DirectoryPath> for DirectoryPathBuf {
    fn as_ref(&self) -> &DirectoryPath {
        self.as_directory_path()
    }
}

impl AsRef<Path> for DirectoryPathBuf {
    fn as_ref(&self) -> &Path {
        self.as_std_path()
    }
}

impl AsRef<str> for DirectoryPathBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A slice of a valid UTF8 path to a directory (akin to str).
///
/// This type supports a number of operations for inspecting a path.
///
/// Note.- Implementation taken from [`Utf8Path`]
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DirectoryPath(Utf8Path);

impl DirectoryPath {
    // invariant: DirectoryPath must be guaranteed to be a valid path (it has been constructed using the [`validate_path`] function)
    #[inline]
    unsafe fn assume_valid(path: &Utf8Path) -> &DirectoryPath {
        // SAFETY: DirectoryPath is marked as #[repr(transparent)] so the conversion from a
        // *const Utf8Path to a *const DirectoryPath is valid.
        &*(path as *const Utf8Path as *const DirectoryPath)
    }

    pub fn as_std_path(&self) -> &Path {
        self.0.as_std_path()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    // pub fn join_filename(&self, filename: impl AsRef<Filename>) -> FilePathBuf {
    //     let filename = Utf8Path::new(filename.as_ref());
    //     FilePathBuf(self.0.join(filename))
    // }
}

impl fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<DirectoryPath> for DirectoryPath {
    fn as_ref(&self) -> &DirectoryPath {
        self
    }
}

impl AsRef<Path> for DirectoryPath {
    fn as_ref(&self) -> &Path {
        self.as_std_path()
    }
}

impl AsRef<str> for DirectoryPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::filename::Filename;
    use crate::FilenameBuf;

    fn take_asref_directory_path(value: impl AsRef<DirectoryPath>, expected: &str) {
        let dir_path: &DirectoryPath = value.as_ref();
        assert_eq!(dir_path.as_str(), expected);
    }

    fn take_asref_std_path(value: impl AsRef<Path>, expected: &str) {
        let dir_path: &Path = value.as_ref();
        assert_eq!(dir_path.to_str().unwrap(), expected);
    }

    fn take_asref_str(value: impl AsRef<str>, expected: &str) {
        let dir_path: &str = value.as_ref();
        assert_eq!(dir_path, expected);
    }

    #[test]
    fn test_directory_path_buf() {
        // valid filenames
        assert!(DirectoryPathBuf::from_str("a/valid/path").is_ok());
        assert!(DirectoryPathBuf::from_str("a/valid/../path").is_ok());
        assert!(DirectoryPathBuf::from_str("").is_ok());
        assert!(DirectoryPathBuf::from_str("valid/..").is_ok());
        assert!(DirectoryPathBuf::from_str("is also/valid path").is_ok());

        // invalid filenames
        assert!(DirectoryPathBuf::from_str("/").is_err());
        assert!(DirectoryPathBuf::from_str("/invalid").is_err());
        assert!(DirectoryPathBuf::from_str("invalid/../..").is_err());
        assert!(DirectoryPathBuf::from_str("../not/valid").is_err());

        // check error type
        let r = DirectoryPathBuf::from_str("/not/valid");
        assert!(matches!(r.unwrap_err(), Error::PathIsRoot));
        let r = DirectoryPathBuf::from_str("not/../../valid/path");
        assert!(matches!(r.unwrap_err(), Error::PathOutsideFilesystem));

        // check AsRef<DirectoryPath> implementations
        let dir_path_buf = DirectoryPathBuf::from_str("a/../valid/path").unwrap();
        take_asref_directory_path(&dir_path_buf, "valid/path");
        take_asref_std_path(&dir_path_buf, "valid/path");
        take_asref_str(&dir_path_buf, "valid/path");
    }

    #[test]
    fn test_directory_path() {
        let directory_path_buf = DirectoryPathBuf::from_str("a/../valid/path").unwrap();
        let directory_path: &DirectoryPath = &directory_path_buf; // this is testing DeRef for FilenameBuf

        take_asref_directory_path(directory_path, "valid/path");
        take_asref_std_path(directory_path, "valid/path");
        take_asref_str(directory_path, "valid/path");
    }
}
