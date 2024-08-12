use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

use crate::Error;

const INVALID_CHARS: &str = "<>:\"\\|?*";

/// Validates the input filename. Main purpose is to check it doesn't contain any `/` (it would
/// become a directory separator)
fn is_valid_filename(filename: &str) -> bool {
    INVALID_CHARS.chars().all(|c| !filename.contains(c)) && filename.chars().all(|c| !std::path::is_separator(c))
}

/// An owned, mutable valid UTF-8 filename name (akin to [`String`]).
///
/// It's guaranteed that the filename contains only valid characters for a filesystem path and it
/// doesn't contain any path separator in it.
#[derive(Debug, Eq, PartialEq, Clone)]
#[repr(transparent)]
pub struct FilenameBuf(String);

impl FilenameBuf {
    #[must_use]
    pub fn as_filename(&self) -> &Filename {
        // SAFETY: every FilenameBuf constructor ensures that self is a valid filename (it has been constructed using the [`is_valid_filename`] function)
        unsafe { Filename::assume_valid(&self.0) }
    }

    /// Creates a new [`FilenameBuf`] from the given `filename` string removing all the invalid
    /// characters. It will error if, after removing invalid characters, it results in an empty string
    pub fn fix_and_create(filename: &str) -> crate::Result<Self> {
        let mut filename = filename.to_string();
        filename.retain(|c| !INVALID_CHARS.contains(c));
        if filename.is_empty() {
            Err(Error::InvalidFilename)
        } else {
            Ok(Self(filename))
        }
    }
}

impl FromStr for FilenameBuf {
    type Err = Error;

    fn from_str(filename: &str) -> std::result::Result<Self, Self::Err> {
        if is_valid_filename(filename) {
            Ok(FilenameBuf(filename.into()))
        } else {
            Err(Error::InvalidFilename)
        }
    }
}

impl Deref for FilenameBuf {
    type Target = Filename;

    fn deref(&self) -> &Filename {
        self.as_filename()
    }
}

impl fmt::Display for FilenameBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<Filename> for FilenameBuf {
    fn as_ref(&self) -> &Filename {
        self.as_filename()
    }
}

impl AsRef<str> for FilenameBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// A slice of a [`FilenameBuf`]: a valid UTF8 filename (akin to [`str`]).
///
/// This type supports a number of operations for inspecting the filename.
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Filename(str);

impl Filename {
    /// Creates a new [`Filename`] from the given string. It won't run any check, so it's up to the
    /// caller to ensure that the given `filename` satisfy all required rules. Use [`FilenameBuf::from_str`]
    /// to create a new instance and execute the rules.
    ///
    /// # Safety
    ///
    /// Given `filename` must be guaranteed to be a valid filename according to rules defined in [`FilenameBuf::from_str`].
    #[inline]
    pub(crate) unsafe fn assume_valid(filename: &str) -> &Filename {
        // SAFETY: FilePath is marked as #[repr(transparent)] so the conversion from a
        // *const Utf8Path to a *const FilePath is valid.
        &*(filename as *const str as *const Filename)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the filename without the extension
    pub fn basename(&self) -> &str {
        match self.0.rsplit_once('.') {
            None => &self.0,
            Some((basename, _)) => basename,
        }
    }

    /// Returns the extension of the filename if it exists
    pub fn extension(&self) -> Option<&str> {
        match self.0.rsplit_once('.') {
            None => None,
            Some((_, ext)) => Some(ext),
        }
    }
}

impl fmt::Display for Filename {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<Filename> for Filename {
    fn as_ref(&self) -> &Filename {
        self
    }
}

impl AsRef<str> for Filename {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn take_asref_filename(value: impl AsRef<Filename>, expected: &str) {
        let filename: &Filename = value.as_ref();
        assert_eq!(filename.as_str(), expected);
    }

    fn take_asref_str(value: impl AsRef<str>, expected: &str) {
        let filename: &str = value.as_ref();
        assert_eq!(filename, expected);
    }

    #[test]
    fn test_filename_buf() {
        // valid filenames
        assert!(FilenameBuf::from_str("valid").is_ok());
        assert!(FilenameBuf::from_str("valid2.txt").is_ok());
        assert!(FilenameBuf::from_str("valid2 [0001+].txt").is_ok());

        // invalid filenames
        assert!(FilenameBuf::from_str("not/valid").is_err());
        assert!(FilenameBuf::from_str("not*valid").is_err());
        assert!(FilenameBuf::from_str("not<valid").is_err());
        assert!(FilenameBuf::from_str("not>valid").is_err());
        assert!(FilenameBuf::from_str("not\\valid").is_err());
        assert!(FilenameBuf::from_str("not\"valid").is_err());
        assert!(FilenameBuf::from_str("not?valid").is_err());
        assert!(FilenameBuf::from_str("not|valid").is_err());

        // check error type
        let r = FilenameBuf::from_str("not/valid");
        assert!(matches!(r.unwrap_err(), Error::InvalidFilename));

        // check AsRef<Filename> implementations
        let filename_buf = FilenameBuf::from_str("valid.filename.txt").unwrap();
        take_asref_filename(&filename_buf, "valid.filename.txt");
        take_asref_str(&filename_buf, "valid.filename.txt");

        assert_eq!(filename_buf.basename(), "valid.filename"); // requires DeRef
        assert_eq!(filename_buf.extension(), Some("txt")); // requires DeRef
    }

    #[test]
    fn test_filename() {
        let filename_buf = FilenameBuf::from_str("valid.filename.txt").unwrap();
        let filename: &Filename = &filename_buf; // this is testing DeRef for FilenameBuf

        assert_eq!(filename.basename(), "valid.filename");
        assert_eq!(filename.extension(), Some("txt"));
        assert_eq!(filename.as_str(), "valid.filename.txt");

        take_asref_filename(filename, "valid.filename.txt");
        take_asref_str(filename, "valid.filename.txt");

        let filename_without_ext = &FilenameBuf::from_str("valid_filename").unwrap();
        assert_eq!(filename_without_ext.extension(), None);
    }
}
