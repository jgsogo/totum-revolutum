use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::Error;
use crate::types::RemotePath;

use super::FileID;

/// A file in pCloud is represented by either a String/path or a FileID
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
    type Error = Error;

    fn try_from(value: RemotePath) -> Result<Self, Self::Error> {
        if !value.to_string().ends_with('/') {
            Ok(File::RemotePath(value))
        } else {
            Err(Error::ParseFileError {
                string: value.to_string(),
            })
        }
    }
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = FileID::from_str(s) {
            Ok(f.into())
        } else if let Ok(p) = RemotePath::from_str(s) {
            p.try_into()
        } else {
            Err(Error::ParseFileError { string: s.to_string() })
        }
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
        assert_eq!(File::from_str("fileid:123")?, FileID(123).into());
        assert_eq!(
            File::from_str("path:/fileid-123")?,
            File::RemotePath(Utf8PathBuf::from_str("/fileid-123")?.try_into()?)
        );

        assert!(File::from_str("path:/path/to/file").is_ok());
        assert!(File::from_str("path:/path/to/folder/").is_err());
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        assert_eq!(&format!("{}", File::FileID(FileID(123))), "fileid:123");
        assert_eq!(
            &format!("{}", File::RemotePath(RemotePath::from_str("path:/fileid-123")?)),
            "path:/fileid-123"
        );
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        assert_eq!(&format!("{:?}", File::FileID(FileID(123))), "FileID(fileid:123)");
        assert_eq!(
            &format!("{:?}", File::RemotePath(RemotePath::from_str("path:/fileid-123")?)),
            "RemotePath(path:\"/fileid-123\")"
        );
        Ok(())
    }
}
