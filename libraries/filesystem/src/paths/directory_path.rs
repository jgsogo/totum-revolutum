use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;

use camino::{Utf8Components, Utf8Path, Utf8PathBuf};

use utils::filesystem::normalize_path;

use crate::paths::filename::Filename;
use crate::{Error, FilePathBuf, Result};

const ROOT_DIR: &str = "";

fn validate_path(path: &Utf8Path) -> Result<Utf8PathBuf> {
    let path: Utf8PathBuf = normalize_path(path);
    if path.starts_with("../") {
        Err(Error::PathOutsideFilesystem)
    } else if path.is_absolute() {
        Err(Error::PathIsRoot)
    } else {
        // Remove trailing slashes and duplicated ones
        let path = path.components().collect();
        Ok(path)
    }
}

/// An owned, mutable valid UTF-8 path to a directory (akin to [`String`]) inside a [`crate::Filesystem`].
///
/// This is always a relative path, normalized (all `..` have been resolved) that doesn't start with
/// `..`, so it's guaranteed that it points to something inside the [`crate::Filesystem`].
///
/// Note.- Implementation taken from [`Utf8PathBuf`].
#[derive(Debug, Eq, PartialEq, Clone)]
#[repr(transparent)]
pub struct DirectoryPathBuf(Utf8PathBuf);

impl DirectoryPathBuf {
    /// Returns the relative path to the root directory: this is just the empty path
    pub fn root() -> DirectoryPathBuf {
        let root = Utf8PathBuf::from(ROOT_DIR);
        DirectoryPathBuf(root)
    }

    /// Extends `self` with `path`.
    ///
    /// The given `path` should follow certain rules so the resulting `DirectoryPathBuf` is still
    /// valid according to internal rules.
    pub fn push(&mut self, path: impl AsRef<Utf8Path>) -> Result<()> {
        let new_path = self.0.join(path);
        let new_path = validate_path(&new_path)?;
        self.0 = new_path;
        Ok(())
    }

    /// Truncates `self` to [`self.parent`].
    ///
    /// Returns `false` and does nothing if [`self.parent`] is already the root directory.
    /// Otherwise, returns `true`.
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    #[must_use]
    pub fn as_directory_path(&self) -> &DirectoryPath {
        // SAFETY: every DirectoryPathBuf constructor ensures that self is valid UTF-8 directory path
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

impl<'a> TryFrom<&'a Utf8Path> for DirectoryPathBuf {
    type Error = Error;

    fn try_from(value: &'a Utf8Path) -> std::result::Result<Self, Self::Error> {
        let value = validate_path(value)?;
        Ok(Self(value))
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

impl AsRef<Utf8Path> for DirectoryPathBuf {
    fn as_ref(&self) -> &Utf8Path {
        self.as_utf8_path()
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

/// A slice of [`DirectoryPathBuf`]: a valid UTF8 path to a directory (akin to [`str`]).
///
/// This type supports a number of operations for inspecting a path.
///
/// Note.- Implementation taken from [`Utf8Path`]
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct DirectoryPath(Utf8Path);

impl DirectoryPath {
    pub(crate) fn root<'a>() -> &'a DirectoryPath {
        unsafe { DirectoryPath::assume_valid(Utf8Path::new(ROOT_DIR)) }
    }

    /// Returns a new [`DirectoryPath`]. The caller is responsible for validating that the input `path`
    /// satisfies all the restrictions. Use [`DirectoryPathBuf::from_str`] to create a new one and
    /// run all the validations.
    ///
    /// # Safety
    ///
    /// Given `path` must be guaranteed to be a valid path according to rules defined by [`DirectoryPathBuf::from_str`].
    #[inline]
    pub unsafe fn assume_valid(path: &Utf8Path) -> &DirectoryPath {
        // SAFETY: DirectoryPath is marked as #[repr(transparent)] so the conversion from a
        // *const Utf8Path to a *const DirectoryPath is valid.
        &*(path as *const Utf8Path as *const DirectoryPath)
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

    /// Produces an iterator over the [`Utf8Components`] of the path.
    ///
    /// Forwards the call to [`Utf8Path::components`]
    pub fn components(&self) -> Utf8Components {
        self.0.components()
    }

    #[must_use]
    pub fn parent(&self) -> Option<&DirectoryPath> {
        self.0.parent().map(|path| {
            // SAFETY: self is valid UTF-8 directory path, so parent is valid UTF-8 directory path as well
            unsafe { DirectoryPath::assume_valid(path) }
        })
    }

    pub fn join_filename(&self, filename: impl AsRef<Filename>) -> FilePathBuf {
        FilePathBuf::new(self, filename)
    }

    pub fn join(&self, path: impl AsRef<DirectoryPath>) -> DirectoryPathBuf {
        let path_buf = self.0.join(path.as_ref());
        DirectoryPathBuf::try_from(path_buf.as_path()).unwrap()
    }
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

impl AsRef<Utf8Path> for DirectoryPath {
    fn as_ref(&self) -> &Utf8Path {
        self.as_utf8_path()
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
    use camino::Utf8Component;

    use crate::FilenameBuf;

    use super::*;

    fn take_asref_directory_path(value: impl AsRef<DirectoryPath>, expected: &str) {
        let dir_path: &DirectoryPath = value.as_ref();
        assert_eq!(dir_path.as_str(), expected);
    }

    fn take_asref_utf8_path(value: impl AsRef<Utf8Path>, expected: &str) {
        let dir_path: &Utf8Path = value.as_ref();
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
        // root
        let root = DirectoryPathBuf::root();
        assert_eq!(root.as_str(), ROOT_DIR);

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
        take_asref_utf8_path(&dir_path_buf, "valid/path");
        take_asref_std_path(&dir_path_buf, "valid/path");
        take_asref_str(&dir_path_buf, "valid/path");

        // push
        let mut path = DirectoryPathBuf::root();
        assert!(path.push("something").is_ok());
        assert_eq!(path.as_str(), "something");
        assert!(path.push("../other").is_ok());
        assert_eq!(path.as_str(), "other");
        assert!(path.push("/fails").is_err());
        assert!(path.push("../../fails").is_err());

        // pop
        assert!(path.pop());
        assert!(!path.pop());
        assert_eq!(path.as_str(), ROOT_DIR);
    }

    #[test]
    fn test_directory_path() {
        let directory_path_buf = DirectoryPathBuf::from_str("a/../valid/path").unwrap();
        let directory_path: &DirectoryPath = &directory_path_buf; // this is testing DeRef for FilenameBuf

        take_asref_directory_path(directory_path, "valid/path");
        take_asref_utf8_path(directory_path, "valid/path");
        take_asref_std_path(directory_path, "valid/path");
        take_asref_str(directory_path, "valid/path");

        let mut components = directory_path.components();
        assert_eq!(components.next(), Some(Utf8Component::Normal("valid")));
        assert_eq!(components.next(), Some(Utf8Component::Normal("path")));
        assert_eq!(components.next(), None);

        // parent
        let path = DirectoryPathBuf::from_str("a/valid/path").unwrap();
        assert_eq!(path.parent().unwrap().as_str(), "a/valid");
        assert_eq!(DirectoryPathBuf::root().parent(), None);

        // join_filename
        let filepath = path.join_filename(FilenameBuf::from_str("filename.txt").unwrap());
        assert_eq!(filepath.as_str(), "a/valid/path/filename.txt");

        // root
        let root = DirectoryPath::root();
        assert_eq!(root.as_str(), ROOT_DIR);
    }

    #[test]
    fn test_trailing_slash() {
        assert_eq!(
            DirectoryPathBuf::from_str("a/../valid/path/").unwrap().as_str(),
            "valid/path"
        );
        assert_eq!(
            DirectoryPathBuf::from_str("valid/path/").unwrap().as_str(),
            "valid/path"
        );
    }

    #[test]
    fn test_extra_slashes() {
        assert_eq!(
            DirectoryPathBuf::from_str("a/..///valid/path").unwrap().as_str(),
            "valid/path"
        );
        assert_eq!(
            DirectoryPathBuf::from_str("valid/path//").unwrap().as_str(),
            "valid/path"
        );
    }
}
