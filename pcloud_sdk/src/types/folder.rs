use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use camino::Utf8PathBuf;

use crate::error::Error;
use crate::types::RemotePath;
use crate::utils::normalize_path;

use super::FolderID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Folder {
    FolderID(FolderID),
    RemotePath(RemotePath),
}

impl FromStr for Folder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = FolderID::from_str(s) {
            return Ok(Folder::FolderID(f));
        } else if let Ok(p) = RemotePath::from_str(s) {
            return Ok(Folder::RemotePath(p));
        }
        Err(Error::ParseFolderError { string: s.to_string() })
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Folder::FolderID(value) => write!(f, "{value}"),
            Folder::RemotePath(value) => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use futures_util::stream::Fold;

    use super::*;

    #[test]
    fn test_parse_str() -> Result<()> {
        assert_eq!(Folder::from_str("folderid:123")?, Folder::FolderID(FolderID(123)));

        let remote_path = Folder::from_str("path:/path/to/something")?;
        assert!(matches!(remote_path, Folder::RemotePath { .. }));
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        assert_eq!(&format!("{}", Folder::FolderID(FolderID(123))), "folderid:123");

        let remote_path = Folder::from_str("path:/path/to/something")?;
        assert_eq!(&format!("{}", remote_path), "path:/path/to/something");
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        assert_eq!(
            &format!("{:?}", Folder::FolderID(FolderID(123))),
            "FolderID(folderid:123)"
        );

        let remote_path = Folder::from_str("path:/path/to/something")?;
        assert_eq!(&format!("{:?}", remote_path), "Path(\"/folderid-123\")");
        Ok(())
    }
}
