use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, StripPrefixError};

use anyhow::{anyhow, bail, Result};
use camino::{Utf8Path, Utf8PathBuf};

use path_utils::{normalize_path, to_absolute_path};

/// Wraps a relative path that has been constructed taking root into account. This path, joined to
/// the `root` used to create it should return the (absolute) original one.  
pub struct LocalPath<'a> {
    path: Utf8PathBuf,
    root: Option<Box<&'a LocalPath<'a>>>,
}

impl<'a> LocalPath<'a> {
    /// Creates a new instance of [`LocalPath`], it normalizes path in `value` and converts it
    /// into an **absolute** path.
    pub fn new_root(value: &Utf8Path) -> Self {
        let path = to_absolute_path(&normalize_path(value));
        Self { path, root: None }
    }

    pub fn is_root(&self) -> bool {
        match &self.root {
            Some(_) => false,
            None => true,
        }
    }

    pub fn new_relative(&'a self, path: &Utf8Path) -> Result<LocalPath<'a>> {
        Ok(Self {
            path: path.to_path_buf(),
            root: Some(Box::new(self)),
        })
    }

    /// Creates a new instance of [`LocalPath`], storing just the relative directory from `root`.
    /// It normalize `path` and preserves trailing `/` if any.
    pub fn try_from(path: &Utf8Path, root: &'a LocalPath) -> Result<LocalPath<'a>> {
        let path = root.path.join(path);
        let path = to_absolute_path(&normalize_path(path));
        match path.strip_prefix(&root.path) {
            Ok(p) => {
                if path.to_string().ends_with("/") {
                    Ok(Self {
                        path: Utf8PathBuf::from(format!("{}/", p)),
                        root: Some(Box::new(root)),
                    })
                } else {
                    Ok(Self {
                        path: p.to_path_buf(),
                        root: Some(Box::new(root)),
                    })
                }
            }
            Err(_) => Err(anyhow!("Path '{path}' is not within the root folder")),
        }
    }

    /// Returns the full path. This is always an **absolute** path
    pub fn full_path(&self) -> Utf8PathBuf {
        match &self.root {
            Some(r) => r.path.join(&self.path),
            None => self.path.clone(),
        }
    }

    pub fn try_exists(&self) -> Result<bool> {
        self.full_path().try_exists().map_err(|e| anyhow!(e))
    }
}

// impl AsRef<Utf8Path> for LocalPath {
//     fn as_ref(&self) -> &Utf8Path {
//         &self.path
//     }
// }
//
// impl AsRef<Path> for LocalPath {
//     fn as_ref(&self) -> &Path {
//         self.path.as_std_path()
//     }
// }
//
// impl AsRef<async_std::path::Path> for LocalPath {
//     fn as_ref(&self) -> &async_std::path::Path {
//         let path: &Path = self.path.as_std_path();
//         path.into()
//     }
// }

impl Display for LocalPath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Debug for LocalPath<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_path())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() -> Result<()> {
        let root = LocalPath::new_root("/".into());
        assert_eq!(root.to_string(), "/");

        let root = LocalPath::new_root("/root".into());
        assert_eq!(root.to_string(), "/root");

        let root = LocalPath::new_root("/root/".into());
        assert_eq!(root.to_string(), "/root/");

        let root = LocalPath::new_root("/root/../other".into());
        assert_eq!(root.to_string(), "/other");

        Ok(())
    }

    #[test]
    fn test_filepaths() -> Result<()> {
        let root = LocalPath::new_root("/root".into());

        {
            let utf8_path = "/root/abs/path/to/file.txt".into();
            let path = LocalPath::try_from(utf8_path, &root)?;
            assert_eq!(path.to_string(), "abs/path/to/file.txt");
            assert_eq!(path.full_path().to_string(), "/root/abs/path/to/file.txt");
        }

        {
            let utf8_path = "/root/abs/../to/file.txt".into();
            let path = LocalPath::try_from(utf8_path, &root)?;
            assert_eq!(path.to_string(), "to/file.txt");
            assert_eq!(path.full_path().to_string(), "/root/to/file.txt");
        }
        Ok(())
    }

    #[test]
    fn test_folders() -> Result<()> {
        let root = LocalPath::new_root("/root/".into());

        {
            let utf8_path = "/root/abs/path/to/folder/".into();
            let path = LocalPath::try_from(utf8_path, &root)?;
            assert_eq!(path.to_string(), "abs/path/to/folder/");
            assert_eq!(path.full_path().to_string(), "/root/abs/path/to/folder/");
        }

        {
            let utf8_path = "/root/abs/../to/folder/".into();
            let path = LocalPath::try_from(utf8_path, &root)?;
            assert_eq!(path.to_string(), "to/folder/");
            assert_eq!(path.full_path().to_string(), "/root/to/folder/");
        }
        Ok(())
    }
}
