use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use fs4::FileExt;
use tracing::debug;

use crate::utils::versioned_data::VersionedData;

pub trait ReadWrite<T> {
    fn read_content(path: &Path) -> std::io::Result<Option<T>> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).expect("Cannot read content from file");

        if !s.is_empty() {
            <Self as ReadWrite<T>>::deserialize(&s).map(|v| Some(v))
        } else {
            Ok(None)
        }
    }

    fn write_content(path: &Path, content: Option<&T>) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Cannot open file");

        if let Some(content) = content {
            let content_str = <Self as ReadWrite<T>>::serialize(content).unwrap();
            f.write_all(content_str.as_bytes()).expect("Error writing the file");
        }
        Ok(())
    }

    fn deserialize(content: &str) -> std::io::Result<T>;
    fn serialize(object: &T) -> std::io::Result<String>;
}

pub struct LockedFile<T>
where
    T: ReadWrite<T>,
{
    path: PathBuf,
    file: File,
    write: bool,

    pub content: T,
}

impl<T> LockedFile<T>
where
    T: ReadWrite<T>,
{
    fn ensure_exists(path: &Path) -> File {
        match File::open(path) {
            Ok(file) => file,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                let parent_dir = path
                    .parent()
                    .expect("Cannot get parent directory from given path. Is it root?");

                std::fs::create_dir_all(parent_dir).expect("Cannot create directory");
                T::write_content(path, None).unwrap();
                File::open(path).unwrap()
            }
            Err(e) => panic!("Unhandled error: {e}"),
        }
    }

    fn read_content(file: File, path: &Path, write: bool) -> Result<Self> {
        debug!("Read file from '{}'", path.display());
        let content = T::read_content(path)?;

        match content {
            None => Err(anyhow!("File is empty, nothing to read")),
            Some(v) => Ok(Self {
                content: v,
                write,
                path: path.to_path_buf(),
                file,
            }),
        }
    }

    pub fn try_read(path: &Path) -> Result<Self> {
        let file = Self::ensure_exists(path);

        debug!("Lock file (shared) '{}'", path.display());
        file.try_lock_shared()?;

        LockedFile::read_content(file, path, false)
    }

    pub fn read(path: &Path) -> Result<Self> {
        let file = Self::ensure_exists(path);

        debug!("Lock file (shared) '{}'", path.display());
        file.lock_shared().unwrap();

        LockedFile::read_content(file, path, false)
    }

    pub fn update(path: &Path) -> Result<Self> {
        let file = Self::ensure_exists(path);

        debug!("Lock file (exclusive) '{}'", path.display());
        file.try_lock_exclusive()?;

        LockedFile::read_content(file, path, true)
    }
}

impl<T> LockedFile<T>
where
    T: Default + ReadWrite<T>,
{
    pub fn update_or_create(path: &Path) -> Result<Self> {
        let file = Self::ensure_exists(path);

        debug!("Lock file (exclusive) '{}'", path.display());
        file.try_lock_exclusive()?;

        debug!("Read file from '{}'", path.display());
        let content = T::read_content(path)?.unwrap_or_default();

        Ok(Self {
            content,
            write: true,
            path: path.to_path_buf(),
            file,
        })
    }
}

impl<T> LockedFile<VersionedData<T>>
where
    VersionedData<T>: ReadWrite<VersionedData<T>>,
{
    pub fn update_or_create(path: &Path, default: T) -> Result<Self> {
        // TODO: Change return type to `Result<(Self, bool)>` so we can know if it was created of updated
        let file = Self::ensure_exists(path);

        debug!("Lock file (exclusive) '{}'", path.display());
        file.try_lock_exclusive()?;

        debug!("Read file from '{}'", path.display());
        let content = VersionedData::<T>::read_content(path)?.unwrap_or_else(|| VersionedData::default(default));

        Ok(Self {
            content,
            write: true,
            path: path.to_path_buf(),
            file,
        })
    }
}

impl<T> Drop for LockedFile<T>
where
    T: ReadWrite<T>,
{
    fn drop(&mut self) {
        if self.write {
            debug!("Save content to file '{}'", self.path.display());
            T::write_content(&self.path, Some(&self.content)).unwrap();
        }
        debug!("Unlock file '{}'", self.path.display());
        self.file.unlock().unwrap();
    }
}
