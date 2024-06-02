use std::fmt::{Display, Formatter};
use std::str::FromStr;

use camino::Utf8PathBuf;

use crate::types::errors::{InvalidFileError, InvalidRemotePathError, InvalidRemotePathKind};
use crate::types::errors::{ParseError, ParseErrorKind};

use super::{FileID, RemotePath};

/// A file in pCloud is represented by either a [`FileID`] or a [`RemotePath`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum File {
    FileID(FileID),
    RemotePath(RemotePath),
}

impl From<FileID> for File {
    fn from(value: FileID) -> Self {
        File::FileID(value)
    }
}

impl TryFrom<RemotePath> for File {
    type Error = InvalidFileError;

    fn try_from(value: RemotePath) -> Result<Self, Self::Error> {
        if !value.to_string().ends_with('/') {
            Ok(File::RemotePath(value))
        } else {
            let source = InvalidRemotePathError {
                source: InvalidRemotePathKind::NotAFile,
            };
            Err(InvalidFileError { source: source.into() })
        }
    }
}

impl TryFrom<Utf8PathBuf> for File {
    type Error = InvalidFileError;

    fn try_from(value: Utf8PathBuf) -> Result<Self, Self::Error> {
        let r: RemotePath = value
            .try_into()
            .map_err(|source: InvalidRemotePathError| InvalidFileError { source: source.into() })?;
        r.try_into()
    }
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            if let Ok(f) = FileID::from_str(s) {
                Ok(f.into())
            } else if let Ok(p) = RemotePath::from_str(s) {
                p.try_into().map_err(ParseErrorKind::InvalidFile)
            } else {
                Err(ParseErrorKind::InvalidFileIDOrRemotePath)
            }
        }
        .map_err(|source| ParseError {
            string: s.to_string(),
            source,
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            File::FileID(value) => write!(f, "{value}"),
            File::RemotePath(value) => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use camino::Utf8PathBuf;

    use super::*;

    #[test]
    fn test_parse_str() -> Result<()> {
        assert_eq!(File::from_str("fileid:123")?, FileID::new(123).into());
        assert_eq!(
            File::from_str("path:/fileid-123")?,
            File::RemotePath(Utf8PathBuf::from_str("/fileid-123")?.try_into()?)
        );

        assert!(File::from_str("path:/path/to/file").is_ok());
        assert!(File::from_str("path:/path/to/folder/").is_err());
        Ok(())
    }

    #[test]
    fn test_parse_errors() {
        let r = File::from_str("fileid:123a");
        assert!(r.is_err());
        assert!(
            matches!(r.unwrap_err(), ParseError {ref string, source: ParseErrorKind::InvalidFileIDOrRemotePath} if string == "fileid:123a")
        );

        let r = File::from_str("path:/invalid/as/file/");
        assert!(r.is_err());
        assert!(
            matches!(r.unwrap_err(), ParseError {ref string, source: ParseErrorKind::InvalidRemotePath(InvalidRemotePathError{source: InvalidRemotePathKind::NotAFile})} if string == "path:/invalid/as/file/")
        );
    }

    #[test]
    fn test_display() -> Result<()> {
        assert_eq!(&format!("{}", File::from(FileID::new(123))), "fileid:123");
        assert_eq!(
            &format!("{}", File::RemotePath(RemotePath::from_str("path:/fileid-123")?)),
            "path:/fileid-123"
        );
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        assert_eq!(&format!("{:?}", File::from(FileID::new(123))), "FileID(fileid:123)");
        assert_eq!(
            &format!("{:?}", File::RemotePath(RemotePath::from_str("path:/fileid-123")?)),
            "RemotePath(path:\"/fileid-123\")"
        );
        Ok(())
    }
}
