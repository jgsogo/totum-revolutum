use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::utils::normalize_path;

const REMOTEPATH_PREFIX: &str = "path";

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RemotePath(pub Utf8PathBuf);

impl Display for RemotePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RemotePath(value) = self;
        write!(f, "{REMOTEPATH_PREFIX}:{value}")
    }
}

impl Debug for RemotePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RemotePath(value) = self;
        write!(f, "{REMOTEPATH_PREFIX}:{value:?}")
    }
}

impl FromStr for RemotePath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s
            .strip_prefix(REMOTEPATH_PREFIX)
            .and_then(|s| s.strip_prefix(':'))
            .ok_or(Error::ParseRemotePathError { string: s.to_string() })?;

        let path = r
            .parse::<Utf8PathBuf>()
            .map_err(|_| Error::ParseRemotePathError { string: s.to_string() })?;

        let path = normalize_path(path);
        if !path.is_absolute() || path.starts_with("/..") {
            return Err(Error::ParseRemotePathError { string: s.to_string() });
        }
        Ok(Self(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remotepath() {
        let remotepath = RemotePath::from_str("path:/path/to/something").unwrap();
        assert_eq!(remotepath.to_string(), "path:/path/to/something");
        assert_eq!(format!("{remotepath:?}"), "path:\"/path/to/something\"");
    }

    #[test]
    fn test_parse_remotepath() {
        assert_eq!(
            RemotePath::from_str("path:/path/to/something").unwrap(),
            RemotePath(Utf8PathBuf::from("/path/to/something"))
        );

        assert_eq!(
            RemotePath::from_str("path:/path/to/../something").unwrap(),
            RemotePath(Utf8PathBuf::from("/path/something"))
        );

        let r = RemotePath::from_str("path-/a/path");
        assert!(r.is_err());

        let r = RemotePath::from_str("path:relative/path");
        assert!(r.is_err());

        let r = RemotePath::from_str("path:/../outside/path");
        assert!(r.is_err());
    }
}
