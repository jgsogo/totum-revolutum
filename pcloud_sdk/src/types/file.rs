use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::Error;
use crate::utils::normalize_path;

use super::FileID;
use camino::Utf8PathBuf;

/// A file in pCloud is represented by either a String/path or a FileID
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum File {
    FileID(FileID),
    Path(Utf8PathBuf),
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = FileID::from_str(s) {
            return Ok(File::FileID(f));
        } else if let Ok(p) = Utf8PathBuf::from_str(s) {
            let p = normalize_path(p);
            if p.starts_with("/") && !p.starts_with("/..") {
                return Ok(File::Path(p));
            }
        }
        Err(Error::ParseFileError { string: s.to_string() })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            File::FileID(value) => write!(f, "{value}"),
            File::Path(value) => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_parse_str() -> Result<()> {
        assert_eq!(File::from_str("fileid:123")?, File::FileID(FileID(123)));
        assert_eq!(
            File::from_str("/fileid-123")?,
            File::Path(Utf8PathBuf::from_str("/fileid-123")?)
        );
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        assert_eq!(&format!("{}", File::FileID(FileID(123))), "fileid:123");
        assert_eq!(
            &format!("{}", File::Path(Utf8PathBuf::from_str("/fileid-123")?)),
            "/fileid-123"
        );
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        assert_eq!(&format!("{:?}", File::FileID(FileID(123))), "FileID(fileid:123)");
        assert_eq!(
            &format!("{:?}", File::Path(Utf8PathBuf::from_str("/fileid-123")?)),
            "Path(\"/fileid-123\")"
        );
        Ok(())
    }
}
