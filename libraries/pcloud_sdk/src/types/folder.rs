use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use camino::Utf8PathBuf;

use crate::types::errors::{InvalidFolderError, InvalidRemotePathError};
use crate::types::errors::{ParseError, ParseErrorKind};

use super::{FolderID, RemotePath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Folder {
    FolderID(FolderID),
    RemotePath(RemotePath),
}

impl From<FolderID> for Folder {
    fn from(value: FolderID) -> Self {
        Folder::FolderID(value)
    }
}

impl From<RemotePath> for Folder {
    fn from(value: RemotePath) -> Self {
        Folder::RemotePath(value)
    }
}

impl TryFrom<Utf8PathBuf> for Folder {
    type Error = InvalidFolderError;

    fn try_from(value: Utf8PathBuf) -> Result<Self, Self::Error> {
        let r: RemotePath = value
            .try_into()
            .map_err(|source: InvalidRemotePathError| InvalidFolderError { source: source.into() })?;
        Ok(r.into())
    }
}

impl FromStr for Folder {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            if let Ok(f) = FolderID::from_str(s) {
                Ok(f.into())
            } else if let Ok(p) = RemotePath::from_str(s) {
                Ok(p.into())
            } else {
                Err(ParseErrorKind::InvalidFolderIDOrRemotePath)
            }
        }
        .map_err(|source| ParseError {
            string: s.to_string(),
            source,
        })
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
    use super::*;
    use crate::types::errors::{InvalidFolderKind, InvalidRemotePathKind};
    use crate::Result;

    #[test]
    fn test_parse_str() -> Result<()> {
        assert_eq!(Folder::from_str("folderid:123")?, FolderID::new(123).into());

        let remote_path = Folder::from_str("path:/path/to/something")?;
        assert!(matches!(remote_path, Folder::RemotePath { .. }));
        Ok(())
    }

    #[test]
    fn test_parse_errors() {
        let r = Folder::from_str("folderid:123a");
        assert!(r.is_err());
        assert!(
            matches!(r.unwrap_err(), ParseError {ref string, source: ParseErrorKind::InvalidFolderIDOrRemotePath} if string == "folderid:123a")
        );

        let path = Utf8PathBuf::from_str("/../rel/path/").unwrap();
        let r = Folder::try_from(path);
        assert!(r.is_err());
        assert!(matches!(
            r.unwrap_err(),
            InvalidFolderError {
                source: InvalidFolderKind::InvalidRemotePath(InvalidRemotePathError {
                    source: InvalidRemotePathKind::OutsideRootFolder
                })
            }
        ));
    }

    #[test]
    fn test_display() -> Result<()> {
        assert_eq!(&format!("{}", Folder::from(FolderID::new(123))), "folderid:123");

        let remote_path = Folder::from_str("path:/path/to/something")?;
        assert_eq!(&format!("{}", remote_path), "path:/path/to/something");
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        assert_eq!(
            &format!("{:?}", Folder::from(FolderID::new(123))),
            "FolderID(folderid:123)"
        );

        let remote_path = Folder::from_str("path:/path/to/something")?;
        assert_eq!(&format!("{:?}", remote_path), "RemotePath(path:\"/path/to/something\")");
        Ok(())
    }
}
