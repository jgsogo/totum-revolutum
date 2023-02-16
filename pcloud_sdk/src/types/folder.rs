use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::Error;
use crate::utils::normalize_path;

use super::FolderID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Folder {
    FolderID(FolderID),
    Path(PathBuf),
}

impl FromStr for Folder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = FolderID::from_str(s) {
            return Ok(Folder::FolderID(f));
        } else if let Ok(p) = PathBuf::from_str(s) {
            let p = normalize_path(p);
            if p.starts_with("/") && !p.starts_with("/..") {
                return Ok(Folder::Path(p));
            }
        }
        Err(Error::ParseFolderError { string: s.to_string() })
    }
}

impl Display for Folder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Folder::FolderID(value) => write!(f, "{value}"),
            Folder::Path(value) => write!(f, "{}", value.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_parse_str() -> Result<()> {
        assert_eq!(Folder::from_str("folderid:123")?, Folder::FolderID(FolderID(123)));
        assert_eq!(
            Folder::from_str("/folderid-123")?,
            Folder::Path(PathBuf::from_str("/folderid-123")?)
        );
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        assert_eq!(&format!("{}", Folder::FolderID(FolderID(123))), "folderid:123");
        assert_eq!(
            &format!("{}", Folder::Path(PathBuf::from_str("/folderid-123")?)),
            "/folderid-123"
        );
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        assert_eq!(
            &format!("{:?}", Folder::FolderID(FolderID(123))),
            "FolderID(folderid:123)"
        );
        assert_eq!(
            &format!("{:?}", Folder::Path(PathBuf::from_str("/folderid-123")?)),
            "Path(\"/folderid-123\")"
        );
        Ok(())
    }
}
