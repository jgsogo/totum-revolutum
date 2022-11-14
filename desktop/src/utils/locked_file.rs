use fs4::FileExt;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tracing::debug;

pub trait ReadWrite<T> {
    fn read_content(path: &Path) -> std::io::Result<T> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s)
            .expect("Cannot read content from file");

        <Self as ReadWrite<T>>::deserialize(&s)
    }

    fn write_content(path: &Path, content: &T) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Cannot open file");

        let content = <Self as ReadWrite<T>>::serialize(content).unwrap();
        f.write_all(content.as_bytes())
            .expect("Error writing the file");
        Ok(())
    }

    fn deserialize(content: &str) -> std::io::Result<T>;
    fn serialize(object: &T) -> std::io::Result<String>;
}

pub struct LockedFile<T>
where
    T: Default + ReadWrite<T>,
{
    path: PathBuf,
    file: File,
    write: bool,

    pub content: T,
}

impl<T> LockedFile<T>
where
    T: Default + ReadWrite<T>,
{
    fn ensure_exists(path: &Path) -> File {
        match File::open(path) {
            Ok(file) => file,
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                let parent_dir = path
                    .parent()
                    .expect("Cannot get parent directory from given path. Is it root?");

                std::fs::create_dir_all(parent_dir).expect("Cannot create directory");
                T::write_content(path, &T::default()).unwrap();
                File::open(path).unwrap()
            }
            Err(e) => panic!("Unhandled error: {e}"),
        }
    }

    pub fn read(path: &Path) -> Self {
        let file = Self::ensure_exists(path);

        debug!("Lock file (shared) '{}'", path.display());
        file.lock_shared().unwrap();

        debug!("Read file from '{}'", path.display());
        let content = T::read_content(path).unwrap();

        Self {
            content,
            write: false,
            path: path.to_path_buf(),
            file,
        }
    }

    pub fn write(path: &Path) -> Self {
        let file = Self::ensure_exists(path);

        debug!("Lock file (exclusive) '{}'", path.display());
        file.lock_exclusive().unwrap();

        debug!("Read file from '{}'", path.display());
        let content = T::read_content(path).unwrap();

        Self {
            content,
            write: true,
            path: path.to_path_buf(),
            file,
        }
    }
}

impl<T> Drop for LockedFile<T>
where
    T: Default + ReadWrite<T>,
{
    fn drop(&mut self) {
        if self.write {
            debug!("Save content to file '{}'", self.path.display());
            T::write_content(&self.path, &self.content).unwrap();
        }
        debug!("Unlock file '{}'", self.path.display());
        self.file.unlock().unwrap();
    }
}
