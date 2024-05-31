use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use camino::{Utf8Components, Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use filesystem::utils::normalize_path;

use crate::types::errors::{InvalidRemotePath, InvalidRemotePathKind};
use crate::types::{ParseError, ParseErrorKind};

const REMOTEPATH_PREFIX: &str = "path:";

/// Contains an **absolute path** representing a remote location in PCloud directory tree. It can be
/// either a file or a folder.
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct RemotePath(Utf8PathBuf);

impl RemotePath {
    pub fn as_path(&self) -> &Utf8Path {
        &self.0
    }

    /// Creates a new [`RemotePath`] by extending `self` with the given `path`. It can fail if the
    /// resulting path doesn't satisfy the constraints of [`RemotePath`].
    ///
    /// Behaviour is the same as of [`Utf8PathBuf::join`]. It's important to note that if `path`
    /// is absolute, it replaces the current path.
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Result<Self, InvalidRemotePath> {
        let path = self.0.join(path);
        path.try_into()
    }

    // #[deprecated(note = "please use `RemotePath::join` instead")]
    pub fn join_with_remote_path(&self, other: &RemotePath) -> Self {
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

    /// Returns the [`Utf8Components`] of the path
    pub fn components(&self) -> Utf8Components {
        self.0.components()
    }
}

impl Display for RemotePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RemotePath(value) = self;
        write!(f, "{REMOTEPATH_PREFIX}{value}")
    }
}

impl Debug for RemotePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let RemotePath(value) = self;
        write!(f, "{REMOTEPATH_PREFIX}{value:?}")
    }
}

impl TryFrom<Utf8PathBuf> for RemotePath {
    type Error = InvalidRemotePath;

    fn try_from(value: Utf8PathBuf) -> Result<Self, Self::Error> {
        RemotePath::try_from(value.as_path())
    }
}

impl TryFrom<&Utf8Path> for RemotePath {
    type Error = InvalidRemotePath;

    fn try_from(value: &Utf8Path) -> Result<Self, Self::Error> {
        let value = normalize_path(value);
        {
            if !value.is_absolute() {
                Err(InvalidRemotePathKind::NoAbsolutePath)
                // ParseErrorKind::InvalidRemotePath("RemotePath only accepts absolute paths")
            } else if value.starts_with("/..") {
                Err(InvalidRemotePathKind::OutsideRootFolder)
            } else {
                Ok(RemotePath(value))
            }
        }
        .map_err(|source| InvalidRemotePath { source })
    }
}

impl FromStr for RemotePath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from_str = || -> Result<_, ParseErrorKind> {
            let s = s
                .strip_prefix(REMOTEPATH_PREFIX)
                .ok_or(ParseErrorKind::NoRemotePathPrefix)?;

            let path = Utf8PathBuf::from_str(s).unwrap();
            path.try_into().map_err(ParseErrorKind::InvalidRemotePath)
        };

        from_str().map_err(|source| ParseError {
            string: s.to_string(),
            source,
        })
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

        assert_eq!(
            "path:/path/to/../something".parse::<RemotePath>().unwrap(),
            RemotePath(Utf8PathBuf::from("/path/something"))
        );

        // Preserves trailing slash (this path can only be used as a folder, never a file)
        assert_eq!(
            RemotePath::from_str("path:/path/with/trailing/slash/")
                .unwrap()
                .to_string(),
            "path:/path/with/trailing/slash/"
        );

        let r = RemotePath::from_str("path-/a/path");
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e.source, ParseErrorKind::NoRemotePathPrefix));
        assert_eq!(
            e.to_string(),
            "Cannot parse from string 'path-/a/path': no RemotePath prefix, missing `path:`"
        );

        let r = RemotePath::from_str("path:relative/path");
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e.source, ParseErrorKind::InvalidRemotePath { .. }));
        assert_eq!(
            e.to_string(),
            "Cannot parse from string 'path:relative/path': InvalidRemotePath no absolute path"
        );

        let r = RemotePath::from_str("path:/../outside/path");
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e.source, ParseErrorKind::InvalidRemotePath { .. }));
        assert_eq!(
            e.to_string(),
            "Cannot parse from string 'path:/../outside/path': InvalidRemotePath resolved path is outside root folder"
        );
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
    fn test_join_with_remote_path() {
        {
            let lhs = RemotePath::from_str("path:/left/hand/side").unwrap();
            let rhs = RemotePath::from_str("path:/rhs").unwrap();
            assert_eq!(lhs.join_with_remote_path(&rhs).to_string(), "path:/left/hand/side/rhs");
        }

        {
            // Trailing on lhs
            let lhs = RemotePath::from_str("path:/left/hand/side/with/trailing/").unwrap();
            let rhs = RemotePath::from_str("path:/rhs").unwrap();
            assert_eq!(
                lhs.join_with_remote_path(&rhs).to_string(),
                "path:/left/hand/side/with/trailing/rhs"
            );
        }

        {
            // Trailing on rhs
            let lhs = RemotePath::from_str("path:/lhs").unwrap();
            let rhs = RemotePath::from_str("path:/rhs/with/trailing/").unwrap();
            assert_eq!(
                lhs.join_with_remote_path(&rhs).to_string(),
                "path:/lhs/rhs/with/trailing/"
            );
        }

        {
            // Weird case -- noop
            let lhs = RemotePath::from_str("path:/").unwrap();
            let rhs = RemotePath::from_str("path:/").unwrap();
            assert_eq!(lhs.join_with_remote_path(&rhs).to_string(), "path:/");
        }
    }

    #[test]
    fn test_join() {
        let lhs = RemotePath::from_str("path:/left/hand/side").unwrap();

        assert_eq!(lhs.join("rhs").unwrap().to_string(), "path:/left/hand/side/rhs");
        assert_eq!(lhs.join("rhs/").unwrap().to_string(), "path:/left/hand/side/rhs/");
        assert_eq!(lhs.join("../rhs").unwrap().to_string(), "path:/left/hand/rhs");
        assert_eq!(lhs.join("../rhs/").unwrap().to_string(), "path:/left/hand/rhs/");
        assert_eq!(lhs.join("/rhs").unwrap().to_string(), "path:/rhs");

        let lhs = RemotePath::from_str("path:/left/hand/side/with/trailing/").unwrap();
        assert_eq!(
            lhs.join("rhs").unwrap().to_string(),
            "path:/left/hand/side/with/trailing/rhs"
        );

        let lhs = RemotePath::from_str("path:/").unwrap();
        assert_eq!(lhs.join("rhs").unwrap().to_string(), "path:/rhs");
        assert_eq!(lhs.join("/").unwrap().to_string(), "path:/");
    }
}
