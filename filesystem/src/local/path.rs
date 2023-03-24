use std::fmt::{Display, Formatter};
use std::path::{Path, StripPrefixError};

use anyhow::{anyhow, bail, Result};
use camino::{Utf8Path, Utf8PathBuf};

use path_utils::{normalize_path, to_absolute_path};

/// Wraps a relative path that has been constructed taking root into account. This path, joined to
/// the `root` used to create it should return the (absolute) original one.  
pub struct LocalPath {
    path: Utf8PathBuf,
    is_root: bool,
}

impl LocalPath {
    /// Creates a new instance of [`LocalPath`], it converts `value` into a normalized absolute path
    pub fn new_root(value: &Utf8Path) -> Self {
        let path = to_absolute_path(&normalize_path(value));
        Self { path, is_root: true }
    }

    pub fn try_from(path: &Utf8Path, root: &LocalPath) -> Result<Self> {
        let path = root.path.join(path);
        let path = to_absolute_path(&normalize_path(path));
        match path.strip_prefix(&root.path) {
            Ok(p) => {
                if path.to_string().ends_with("/") {
                    Ok(Self {
                        path: Utf8PathBuf::from(format!("{}/", p)),
                        is_root: false,
                    })
                } else {
                    Ok(Self {
                        path: p.to_path_buf(),
                        is_root: false,
                    })
                }
            }
            Err(_) => Err(anyhow!("Path '{path}' is not within the root folder")),
        }
    }

    pub fn join(&self, other: &LocalPath) -> Result<LocalPath> {
        if other.is_root {
            bail!("Cannot join with a root path. `{}` is a root one", other.path)
        }
        Ok(Self {
            path: self.path.join(&other.path),
            is_root: self.is_root,
        })
    }

    pub fn try_exists(&self) -> Result<bool> {
        if !self.is_root {
            bail!("Cannot check existence of a non-root path")
        }
        self.path.try_exists().map_err(|e| anyhow!(e))
    }
}

impl AsRef<Utf8Path> for LocalPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.path
    }
}

impl AsRef<Path> for LocalPath {
    fn as_ref(&self) -> &Path {
        self.path.as_std_path()
    }
}

impl AsRef<async_std::path::Path> for LocalPath {
    fn as_ref(&self) -> &async_std::path::Path {
        let path: &Path = self.path.as_std_path();
        path.into()
    }
}

impl Display for LocalPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
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
            assert_eq!(root.join(&path)?.to_string(), "/root/abs/path/to/file.txt");
        }

        {
            let utf8_path = "/root/abs/../to/file.txt".into();
            let path = LocalPath::try_from(utf8_path, &root)?;
            assert_eq!(path.to_string(), "to/file.txt");
            assert_eq!(root.join(&path)?.to_string(), "/root/to/file.txt");
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
            assert_eq!(root.join(&path)?.to_string(), "/root/abs/path/to/folder/");
        }

        {
            let utf8_path = "/root/abs/../to/folder/".into();
            let path = LocalPath::try_from(utf8_path, &root)?;
            assert_eq!(path.to_string(), "to/folder/");
            assert_eq!(root.join(&path)?.to_string(), "/root/to/folder/");
        }
        Ok(())
    }
}
