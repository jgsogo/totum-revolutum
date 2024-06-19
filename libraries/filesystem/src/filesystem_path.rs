use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

use camino::{Utf8Path, Utf8PathBuf};

use utils::filesystem::normalize_path;

use crate::{Error, Result};

const ROOT_DIR: &str = "";

fn validate_path(path: &Utf8Path) -> Result<Utf8PathBuf> {
    let path = normalize_path(path);
    if path.starts_with("../") {
        Err(Error::PathOutsideFilesystem)
    } else {
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
    pub fn as_path(&self) -> &DirectoryPath {
        // SAFETY: every Utf8PathBuf constructor ensures that self is valid UTF-8
        unsafe { DirectoryPath::assume_valid(&self.0) }
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

    pub fn join_filename(&self, filename: impl AsRef<Filename>) -> FilePathBuf {
        let filename = Utf8Path::new(filename.as_ref());
        FilePathBuf(self.0.join(filename))
    }
}

/// An owned, mutable valid UTF-8 path to a file (akin to [`String`]).
///
/// Note.- Implementation taken from [`Utf8PathBuf`].
#[derive(Debug, Eq, PartialEq, Clone)]
#[repr(transparent)]
pub struct FilePathBuf(Utf8PathBuf);

impl FilePathBuf {
    #[must_use]
    pub fn as_path(&self) -> &FilePath {
        // SAFETY: every FilePathBuf constructor ensures that self is a valid path (it has been constructed using the [`validate_path`] function)
        unsafe { FilePath::assume_valid(&self.0) }
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
            .unwrap_or(unsafe { DirectoryPath::assume_valid(Utf8Path::new(ROOT_DIR)) })
    }

    pub fn filename(&self) -> &str {
        self.0.file_name().unwrap()
    }
}

// ---
// AsRef impls
// ---

impl AsRef<DirectoryPath> for DirectoryPath {
    fn as_ref(&self) -> &DirectoryPath {
        self
    }
}

impl AsRef<DirectoryPath> for DirectoryPathBuf {
    fn as_ref(&self) -> &DirectoryPath {
        self.as_path()
    }
}

impl AsRef<FilePath> for FilePath {
    fn as_ref(&self) -> &FilePath {
        self
    }
}

impl AsRef<FilePath> for FilePathBuf {
    fn as_ref(&self) -> &FilePath {
        self.as_path()
    }
}
