use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use flume::Sender;
use tokio::sync::oneshot::Receiver;

use crate::{Error, Result};
use crate::{File, FileMetadata, Filesystem};

const ROOT_DIRECTORY: &str = "";

pub trait FilesystemIndexedDbFile {
    /// The filename of this file
    fn filename(&self) -> &str;

    /// The size (bytes) of this file
    fn size(&self) -> u64;

    /// The sha256 hash of the file
    fn hash(&self) -> &str;
}

pub trait FilesystemIndexedDbDirectory {
    /// Full path of the directory (relative to the root)
    fn full_path(&self) -> &Utf8Path;
}

/// Defines the methods that any type should implement, so it can play like the _index_ in the
/// [super::FilesystemIndexed] composite. Anything implementing this trait will have a blanket
/// implementation of the [`Filesystem`] trait
pub trait FilesystemIndexedDatabase: Send + Sync {
    type File: FilesystemIndexedDbFile;
    type Directory: FilesystemIndexedDbDirectory;

    /// Returns an iterator with all the [`Self::File`] stored in the database
    fn all_files(&self) -> Result<impl Iterator<Item = Self::File>>;

    /// Returns an iterator with all the [`Self::Directory`] stored in the database
    fn all_directories(&self) -> Result<impl Iterator<Item = Self::Directory>>;

    /// Returns an iterator with all the files inside a given [`Self::Directory`]
    fn get_files_in_directory(&self, dir: &Self::Directory) -> Result<impl Iterator<Item = Self::File>>;

    /// Returns a [`Self::Directory`] given its path.
    fn get_directory(&self, path: &Utf8Path) -> Result<Self::Directory>;

    /// Returns a [`Self::File`] inside a [`Self::Directory`] given its `filename`
    fn get_file(&self, dir: &Self::Directory, filename: &str) -> Result<Self::File>;

    /// Returns or creates a new [`Self::Directory`]. The caller can pass the parent directory
    /// as an optimization, so we don't need to hit the database to get it.
    fn get_or_create_directory(
        &self,
        path: &Utf8Path,
        parent_dir: Option<Self::Directory>,
    ) -> Result<(Self::Directory, bool)>;

    /// Deletes the given directory
    fn delete_file(&self, file: Self::File) -> Result<()>;

    /// Returns the [`Self::Directory`]s that are direct children of the given directory.
    fn get_directories_in_directory(&self, dir: &Self::Directory) -> Result<impl Iterator<Item = Self::Directory>>;

    /// Deletes the given directory
    fn delete_directory(&self, dir: Self::Directory) -> Result<()>;

    /// Deletes the given directory and every file and children directory contained inside. This
    /// function is recursive.
    fn delete_directory_on_cascade(&self, dir: Self::Directory) -> Result<()>;

    /// Inserts or updates a [`Self::File`] entry. It uses the given given directory `dir` and
    /// `filename` to search for the file and updates (or inserts) the row with the values from
    /// `new_size` and `new_hash`.
    fn upsert_file(
        &self,
        dir: &Self::Directory,
        filename: &str,
        new_size: Option<u64>,
        new_hash: Option<&str>,
    ) -> Result<()>;

    /// Updates the given [`Self::File`] with the values provided. Returns the updated [`Self::File`]
    fn update_file(
        &self,
        file: Self::File,
        new_directory: Option<&Self::Directory>,
        new_filename: Option<&str>,
        new_size: Option<u64>,
        new_hash: Option<&str>,
    ) -> Result<Self::File>;
}

#[async_trait]
impl<T: FilesystemIndexedDatabase> Filesystem for T {
    async fn sync_all(self) -> Result<()> {
        Ok(())
    }

    async fn walk_directory(
        &self,
        tx: Sender<Box<dyn FileMetadata>>,
        _threads: usize,
        _custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        for dir in self.all_directories()? {
            for file in self.get_files_in_directory(&dir)? {
                let file_wrapper = FileWrapper::from(&dir, &file);
                tx.send(Box::new(file_wrapper))
                    .map_err(|e| Error::Other(e.to_string()))?;
            }
        }
        Ok(())
    }

    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        let path = self.check_path(path)?;
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;

        let dir = self.get_directory(dirname)?;
        let file = self.get_file(&dir, filename)?;
        Ok(Box::new(FileWrapper::from(&dir, &file)))
    }

    /// Returns if the given `path` corresponds to a file
    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        match self.get_metadata(path).await {
            Ok(_) => Ok(true),
            Err(e) => match e {
                Error::PathDoesNotExist => Ok(false),
                _ => Err(e),
            },
        }
    }

    /// Forbidden. A database cannot open a file.
    async fn open(&self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        Err(Error::Forbidden)
    }

    /// Forbidden. A database cannot create a file.
    async fn create(&mut self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        Err(Error::Forbidden)
    }

    async fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        let path = self.check_path(path)?;
        let mut current_path = Utf8Path::new(ROOT_DIRECTORY).to_path_buf();
        let mut parent_dir = Some(self.get_directory(&current_path)?);

        for it in path.components() {
            if let Utf8Component::Normal(c) = it {
                current_path = current_path.join(c);
                let (dir, _) = self.get_or_create_directory(&current_path, parent_dir)?;
                parent_dir = Some(dir);
            }
        }
        Ok(())
    }

    async fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        let path = self.check_path(path)?;
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;

        let dir = self.get_directory(dirname)?;
        let file = self.get_file(&dir, filename)?;
        self.delete_file(file)
    }

    async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
        let path = self.check_path(path)?;

        let dir = self.get_directory(&path)?;

        // Can't delete if it contains files
        if self.get_files_in_directory(&dir)?.next().is_some() {
            return Err(Error::NotEmptyDirectory);
        }

        // Can't delete if it contains children directories
        if self.get_directories_in_directory(&dir)?.next().is_some() {
            return Err(Error::NotEmptyDirectory);
        }

        self.delete_directory(dir)
    }

    async fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        let path = self.check_path(path)?;
        let dir = self.get_directory(&path)?;
        self.delete_directory_on_cascade(dir)
    }

    async fn internal_copy(
        &mut self,
        origin: &Utf8Path,
        target: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        let target = self.check_path(target)?;
        let origin = self.check_path(origin)?;

        let target_filename = target
            .file_name()
            .ok_or(Error::Other("Filename expected".to_string()))?;

        // Check that target directory exists
        let target_dir = {
            let target_dirname = target.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
            self.get_directory(target_dirname)?
        };

        // If not force, check target file doesn't exist
        match self.get_file(&target_dir, target_filename) {
            Ok(_) => {
                if !force {
                    return Err(Error::TargetFileExists);
                }
            }
            Err(e) => match e {
                Error::PathDoesNotExist => {}
                _ => {
                    return Err(e);
                }
            },
        }

        let origin_file = {
            let origin_dirname = origin.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
            let origin_filename = origin
                .file_name()
                .ok_or(Error::Other("Filename expected".to_string()))?;
            let origin_dir = self.get_directory(origin_dirname)?;
            self.get_file(&origin_dir, origin_filename)?
        };

        self.upsert_file(
            &target_dir,
            target_filename,
            Some(origin_file.size()),
            Some(origin_file.hash()),
        )?;
        Ok(None)
    }

    async fn internal_move(
        &mut self,
        origin: &Utf8Path,
        target: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        let target = self.check_path(target)?;
        let origin = self.check_path(origin)?;

        let target_filename = target
            .file_name()
            .ok_or(Error::Other("Filename expected".to_string()))?;

        // Check that target directory exists
        let target_dir = {
            let target_dirname = target.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
            self.get_directory(target_dirname)?
        };

        let file = self.get_file(&target_dir, target_filename);
        match file {
            // If the target file exists, remove it (if `force`), otherwise return error
            Ok(file) => {
                if force {
                    self.delete_file(file)?;
                } else {
                    return Err(Error::TargetFileExists);
                }
            }
            // If there is an error, forward it unless it's a `PathDoesNotExist`
            Err(e) => match e {
                Error::PathDoesNotExist => {}
                _ => {
                    return Err(e);
                }
            },
        }

        // Update the origin according to the new target values
        let origin_dirname = origin.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let origin_dir = self.get_directory(origin_dirname)?;
        let origin_filename = origin
            .file_name()
            .ok_or(Error::Other("Filename expected".to_string()))?;
        let origin_file = self.get_file(&origin_dir, origin_filename)?;
        let _ = self.update_file(origin_file, Some(&target_dir), Some(target_filename), None, None)?;
        Ok(None)
    }
}

#[derive(Debug)]
struct FileWrapper {
    path: Utf8PathBuf,
    size: u64,
    hash: String,
}

impl FileWrapper {
    pub fn from<D: FilesystemIndexedDbDirectory, F: FilesystemIndexedDbFile>(dir: &D, file: &F) -> Self {
        let path = dir.full_path().join(file.filename());
        Self {
            path,
            size: file.size(),
            hash: file.hash().to_string(),
        }
    }
}

impl FileMetadata for FileWrapper {
    fn path(&self) -> &Utf8Path {
        &self.path
    }

    fn size(&self) -> Result<u64> {
        Ok(self.size)
    }

    fn hash(&self) -> Result<String> {
        Ok(self.hash.clone())
    }
}
