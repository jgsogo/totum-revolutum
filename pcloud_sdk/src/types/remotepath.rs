use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use camino::{Utf8Components, Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::utils::normalize_path;

const REMOTEPATH_PREFIX: &str = "path";

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
/// Contains an absolute path representing a remote location in pcloud directory tree.
pub struct RemotePath(Utf8PathBuf);

impl RemotePath {
    pub fn path(&self) -> &Utf8Path {
        &self.0
    }

    pub fn join(&self, other: &RemotePath) -> Self {
        let has_trailing = {
            let other_str = other.0.to_string();
            other_str != "/" && other_str.ends_with('/')
        };

        // We need to remove the leading `/` from `other`, then a regular `join` works
        let other = other.0.strip_prefix("/").unwrap();
        let joined_paths = self.0.join(other);
        if has_trailing {
            Self(Utf8PathBuf::from(format!("{}/", joined_paths)))
        } else {
            Self(joined_paths)
        }
    }

    pub fn components(&self) -> Utf8Components {
        self.0.components()
    }
}

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

impl TryFrom<Utf8PathBuf> for RemotePath {
    type Error = Error;

    fn try_from(value: Utf8PathBuf) -> Result<Self, Self::Error> {
        RemotePath::try_from(value.as_path())
    }
}

impl TryFrom<&Utf8Path> for RemotePath {
    type Error = Error;

    fn try_from(value: &Utf8Path) -> Result<Self, Self::Error> {
        let value = normalize_path(value);
        if !value.is_absolute() || value.starts_with("/..") {
            return Err(Error::ParseRemotePathError {
                string: value.to_string(),
            });
        }
        Ok(RemotePath(value))
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

        // Preserves trailing slash (this path can only we used as a folder, never a file)
        assert_eq!(
            RemotePath::from_str("path:/path/with/trailing/slash/")
                .unwrap()
                .to_string(),
            "path:/path/with/trailing/slash/"
        );

        let r = RemotePath::from_str("path-/a/path");
        assert!(r.is_err());

        let r = RemotePath::from_str("path:relative/path");
        assert!(r.is_err());

        let r = RemotePath::from_str("path:/../outside/path");
        assert!(r.is_err());
    }

    #[test]
    fn test_from_utf8path() {
        assert_eq!(
            RemotePath::try_from(Utf8PathBuf::from("/a/path")).unwrap().to_string(),
            "path:/a/path"
        );

        assert_eq!(
            RemotePath::try_from(Utf8PathBuf::from("/a/another/../path"))
                .unwrap()
                .to_string(),
            "path:/a/path"
        );

        assert!(RemotePath::try_from(Utf8PathBuf::from("/../path")).is_err());
        assert!(RemotePath::try_from(Utf8PathBuf::from("../path")).is_err());
    }

    #[test]
    fn test_join() {
        {
            let lhs = RemotePath::from_str("path:/left/hand/side").unwrap();
            let rhs = RemotePath::from_str("path:/rhs").unwrap();
            assert_eq!(lhs.join(&rhs).to_string(), "path:/left/hand/side/rhs");
        }

        {
            // Trailing on lhs
            let lhs = RemotePath::from_str("path:/left/hand/side/with/trailing/").unwrap();
            let rhs = RemotePath::from_str("path:/rhs").unwrap();
            assert_eq!(lhs.join(&rhs).to_string(), "path:/left/hand/side/with/trailing/rhs");
        }

        {
            // Trailing on rhs
            let lhs = RemotePath::from_str("path:/lhs").unwrap();
            let rhs = RemotePath::from_str("path:/rhs/with/trailing/").unwrap();
            assert_eq!(lhs.join(&rhs).to_string(), "path:/lhs/rhs/with/trailing/");
        }

        {
            // Weird case -- noop
            let lhs = RemotePath::from_str("path:/").unwrap();
            let rhs = RemotePath::from_str("path:/").unwrap();
            assert_eq!(lhs.join(&rhs).to_string(), "path:/");
        }
    }
}
