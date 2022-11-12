use fs4::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::ErrorKind;
use std::ops::Drop;
use std::path::{Path, PathBuf};
use tracing::debug;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO(low): Logic relate to confy (load/store) has nothing to do with locking

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct VersionedData<T> {
    version: String,
    pub data: T,
}

impl<T: Default> Default for VersionedData<T> {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            data: T::default(),
        }
    }
}

pub struct LockedFile<T>
where
    T: std::fmt::Debug + Serialize,
{
    data: VersionedData<T>,
    path: PathBuf,
    file: File,
    write: bool,
}

impl<T> LockedFile<T>
where
    T: Default + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug,
{
    pub fn data(&self) -> &T {
        &self.data.data
    }

    pub fn data_as_mut(&mut self) -> &mut T {
        assert!(self.write, "Write is required to borrow mut");
        &mut self.data.data
    }

    fn ensure_exists(path: &Path) -> File {
        match File::open(path) {
            Ok(file) => file,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                std::fs::create_dir_all(path.parent().unwrap())
                    .expect("Cannot create home directory");
                confy::store_path(path, VersionedData::<T>::default()).unwrap();
                File::open(path).unwrap()
            }
            Err(e) => panic!("Unhandled error: {e}"),
        }
    }
}

pub trait LockedFileTrait {
    fn read(path: &Path) -> Self;
    fn write(path: &Path) -> Self;
}

impl<T> LockedFileTrait for LockedFile<T>
where
    T: Default + Serialize + for<'a> Deserialize<'a> + std::fmt::Debug,
{
    fn read(path: &Path) -> Self {
        let file = Self::ensure_exists(path);

        debug!("Lock file (shared) '{}'", path.display());
        file.lock_shared().unwrap();

        debug!("Read file from '{}'", path.display());
        let cfg: VersionedData<T> = confy::load_path(path)
            .unwrap_or_else(|_| panic!("Failed to open '{}' file", path.display()));

        Self {
            data: cfg,
            write: false,
            path: path.to_path_buf(),
            file,
        }
    }

    fn write(path: &Path) -> Self {
        let file = Self::ensure_exists(path);

        debug!("Lock file (exclusive) '{}'", path.display());
        file.lock_exclusive().unwrap();

        debug!("Read file from '{}'", path.display());
        let cfg: VersionedData<T> = confy::load_path(path)
            .unwrap_or_else(|_| panic!("Failed to open '{}' file", path.display()));

        Self {
            data: cfg,
            write: true,
            path: path.to_path_buf(),
            file,
        }
    }
}

impl<T: std::fmt::Debug + Serialize> Drop for LockedFile<T> {
    fn drop(&mut self) {
        if self.write {
            debug!(
                "Save content to file '{}': {:?}",
                self.path.display(),
                self.data
            );
            confy::store_path(&self.path, &self.data).unwrap();
        }
        debug!("Unlock file '{}'", self.path.display());
        self.file.unlock().unwrap();
    }
}
