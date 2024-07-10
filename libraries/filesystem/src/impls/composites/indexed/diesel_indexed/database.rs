use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use diesel_utils::managers::AllManager;

use crate::impls::composites::indexed::diesel_indexed::models;
use crate::impls::composites::{FilesystemIndexedDatabase, FilesystemIndexedDbDirectory, FilesystemIndexedDbFile};
use crate::{DirectoryPath, DirectoryPathBuf, Error, Filename, Result};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/impls/composites/indexed/diesel_indexed/migrations");

/// Implementation of the [`FilesystemIndexedDatabase`] trait using a sqlite3 database and the
/// models defined in [`models::File`] and [`models::Directory`]. To be used out-of-the-box for the
/// [`super::FilesystemIndexed`] composite implementation
#[derive(Clone)]
pub struct DatabaseImpl {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseImpl {
    pub fn new(database_url: &str) -> Result<Self> {
        let pool = {
            let manager = ConnectionManager::<SqliteConnection>::new(database_url);
            Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .map_err(|e| Error::Other(format!("Failed to create the connection pool: {e}")))?
        };

        let mut conn = pool
            .get()
            .map_err(|e| Error::Other(format!("Failed to get one connection from the pool: {e}")))?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| Error::Other(format!("Failed to connect to DB: {e}")))?;

        Ok(Self::new_from_connection(pool))
    }

    pub fn new_from_connection(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }

    pub fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| Error::Other(format!("Failed to get one connection from the pool: {e}")))?;

        // We need to enable foreign_keys per connection. This will execute this statement several
        // times if the connections are reused.
        diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(conn)
    }

    fn create_directory(
        &self,
        dirname: &DirectoryPath,
        parent_dir: Option<&models::Directory>,
    ) -> Result<models::Directory> {
        let parent_dirname = dirname.parent().unwrap_or_else(|| DirectoryPath::root());

        let parent_dir_id = match parent_dir {
            None => {
                let parent_dir = self.get_directory(parent_dirname)?;
                parent_dir.id
            }
            Some(p) => {
                if parent_dirname != p.full_path() {
                    return Err(Error::Other(
                        "Given parent_dir is not a parent of the directory we are creating".to_string(),
                    ));
                }
                p.id
            }
        };

        use super::schema::directories::dsl::*;
        let mut conn = self.get_conn()?;
        let new_directory = models::NewDirectory {
            parent_id: Some(parent_dir_id),
            full_path: dirname.as_str(),
        };
        let dir = diesel::insert_into(directories)
            .values(&new_directory)
            .returning(models::Directory::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(dir)
    }
}

impl FilesystemIndexedDatabase for DatabaseImpl {
    type File = models::File;
    type Directory = models::Directory;

    fn all_files(&self) -> Result<impl Iterator<Item = Self::File>> {
        let mut conn = self.get_conn()?;
        let all_files = models::File::all(models::File::as_select(), &mut conn)?;
        let all_files = all_files.collect::<Vec<_>>();
        Ok(all_files.into_iter())
    }

    fn all_directories(&self) -> Result<impl Iterator<Item = Self::Directory>> {
        let mut conn = self.get_conn()?;
        let all_directories = models::Directory::all(models::Directory::as_select(), &mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        let all_directories = all_directories.collect::<Vec<_>>();
        Ok(all_directories.into_iter())
    }

    fn get_files_in_directory(&self, dir: &Self::Directory) -> Result<impl Iterator<Item = Self::File>> {
        use super::schema::files::dsl::*;
        let mut conn = self.get_conn()?;
        let files_in_dir: Vec<models::File> = files
            .filter(directory_id.eq(dir.id))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(files_in_dir.into_iter())
    }

    fn get_directory(&self, path: &DirectoryPath) -> Result<Self::Directory> {
        use super::schema::directories::dsl::*;
        let mut conn = self.get_conn()?;
        match directories
            .filter(full_path.eq(path.to_string()))
            .select(models::Directory::as_select())
            .first(&mut conn)
        {
            Ok(record) => Ok(record),
            Err(e) => match e {
                diesel::result::Error::NotFound => Err(Error::PathDoesNotExist),
                _ => Err(Error::Other(e.to_string())),
            },
        }
    }

    fn get_file(&self, dir: &Self::Directory, filename: &Filename) -> Result<Self::File> {
        use super::schema::files::dsl::*;

        let mut conn = self.get_conn()?;
        let files_found = files
            .filter(name.eq(filename.as_str()).and(directory_id.eq(dir.id)))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        if files_found.is_empty() {
            Err(Error::PathDoesNotExist)
        } else {
            Ok(files_found.into_iter().nth(0).unwrap())
        }
    }

    fn get_or_create_directory(
        &self,
        path: &DirectoryPath,
        parent_dir: Option<&Self::Directory>,
    ) -> Result<(Self::Directory, bool)> {
        // Try to get, if found return (any error other than not-found is returned too)
        match self.get_directory(path) {
            Ok(dir) => {
                return Ok((dir, false));
            }
            Err(e) => match e {
                Error::PathDoesNotExist => {}
                _ => {
                    return Err(Error::Other(e.to_string()));
                }
            },
        }

        // Not found, we need to create it
        let r = self.create_directory(path, parent_dir)?;
        Ok((r, true))
    }

    fn get_or_create_directory_all(&self, path: &DirectoryPath) -> Result<(Self::Directory, bool)> {
        // Get the closest parent that exists
        let mut current_lookup = path;
        let mut children = Vec::new();
        let mut parent_found: Option<Self::Directory>;
        loop {
            match self.get_directory(current_lookup) {
                Ok(found) => {
                    parent_found = Some(found);
                    break;
                }
                Err(e) => match e {
                    Error::PathDoesNotExist => {
                        let (parent, child) = current_lookup.split_parent();
                        let parent = parent.unwrap_or_else(|| DirectoryPath::root());
                        children.push(child.unwrap());

                        // Try with parent
                        current_lookup = parent;
                    }
                    _ => {
                        return Err(Error::Other(e.to_string()));
                    }
                },
            }
        }

        // It was already created
        if current_lookup == path {
            return Ok((parent_found.unwrap(), false));
        }

        // We have the closest parent that exists (`current_lookup`), now we need to create all the
        // directories up to the final path
        let mut path: DirectoryPathBuf = current_lookup.into();
        while let Some(child) = children.pop() {
            path = path.join(child);
            parent_found = Some(self.create_directory(&path, parent_found.as_ref())?);
        }

        Ok((parent_found.unwrap(), true))
    }

    fn delete_file(&self, file: Self::File) -> Result<()> {
        use super::schema::files::dsl::*;
        let mut conn = self.get_conn()?;
        diesel::delete(files.filter(id.eq(file.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn get_directories_in_directory(&self, dir: &Self::Directory) -> Result<impl Iterator<Item = Self::Directory>> {
        use super::schema::directories::dsl::*;
        let mut conn = self.get_conn()?;
        let directories_in_dir: Vec<models::Directory> = directories
            .filter(parent_id.eq(dir.id))
            .select(models::Directory::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(directories_in_dir.into_iter())
    }

    fn delete_directory(&self, dir: Self::Directory) -> Result<()> {
        use super::schema::directories::dsl::*;
        let mut conn = self.get_conn()?;
        diesel::delete(directories.filter(id.eq(dir.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn delete_directory_on_cascade(&self, dir: Self::Directory) -> Result<()> {
        // TODO: Detect if DELETE ON CASCADE if activated for this DB
        self.delete_directory(dir)
    }

    fn upsert_file(
        &self,
        dir: &Self::Directory,
        filename: &Filename,
        new_size: Option<u64>,
        new_hash: Option<&str>,
    ) -> Result<()> {
        // Get the size and hash, either from the given value or the existing file
        let (size_, hash_) = match (new_size, new_hash) {
            (Some(size_), Some(hash_)) => (size_, hash_.to_string()),
            _ => match self.get_file(dir, filename) {
                Ok(file) => (
                    new_size.unwrap_or(file.size()),
                    new_hash.unwrap_or(file.hash()).to_string(),
                ),
                Err(e) => match e {
                    Error::PathDoesNotExist => {
                        return Err(Error::Other(
                            "If the target file doesn't exist, both values size and hash need to be provided"
                                .to_string(),
                        ));
                    }
                    _ => {
                        return Err(Error::Other(e.to_string()));
                    }
                },
            },
        };

        let new_file = models::NewFile {
            name: filename.as_str(),
            directory_id: dir.id,
            hash: &hash_,
            size: size_ as i32,
        };

        // let changes = (hash.eq(hash_), size.eq(size_ as i32));

        use super::schema::files::dsl::*;
        let mut conn = self.get_conn()?;
        diesel::insert_into(files)
            .values(&new_file)
            .on_conflict((name, directory_id))
            .do_update()
            .set(&new_file)
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn update_file(
        &self,
        file: Self::File,
        new_directory: Option<&Self::Directory>,
        new_filename: Option<&Filename>,
        new_size: Option<u64>,
        new_hash: Option<&str>,
    ) -> Result<Self::File> {
        let new_file = models::NewFile {
            name: new_filename.map(|v| v.as_str()).unwrap_or(file.filename().as_str()),
            directory_id: new_directory.map(|v| v.id).unwrap_or(file.directory_id),
            hash: new_hash.unwrap_or(file.hash()),
            size: new_size.unwrap_or(file.size()) as i32,
        };

        let mut conn = self.get_conn()?;
        use super::schema::files::dsl::*;
        let file_updated = diesel::update(files)
            .filter(id.eq(file.id))
            .set(&new_file)
            .returning(models::File::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(file_updated)
    }

    fn create_file(&self, dir: &Self::Directory, filename: &Filename, hash_: &str, size_: i32) -> Result<Self::File> {
        use super::schema::files::dsl::*;
        let mut conn = self.get_conn()?;

        let new_file = models::NewFile {
            name: filename.as_str(),
            directory_id: dir.id,
            hash: hash_,
            size: size_,
        };
        let file = diesel::insert_into(files)
            .values(&new_file)
            .returning(models::File::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use camino_tempfile::NamedUtf8TempFile;
    use std::str::FromStr;

    use ignore_files::IgnoreFilter;

    use crate::impls::FilesystemLocalTemp;
    use crate::{DirectoryPath, DirectoryPathBuf, FilePathBuf, FilenameBuf, Filesystem, FilesystemOps};

    use super::*;

    /// Creates a directory and return it
    async fn create_dir_all(db: &mut DatabaseImpl, path: &DirectoryPath) -> models::Directory {
        db.create_dir_all(path).await.unwrap();
        db.get_directory(path).unwrap()
    }

    /// Populates the given database with some data
    async fn populate_db(db: &mut DatabaseImpl) {
        // Some directories
        let filename1: &Filename = &FilenameBuf::from_str("file1.txt").unwrap();
        let filename2: &Filename = &FilenameBuf::from_str("file2.txt").unwrap();

        {
            let dir = DirectoryPathBuf::from_str("dir1/subdir1/subsubdir1").unwrap();
            let dir = create_dir_all(db, &dir).await;
            db.create_file(&dir, filename1, "file1", 1).unwrap();
            db.create_file(&dir, filename2, "file2", 2).unwrap();
        }
        {
            let dir = DirectoryPathBuf::from_str("dir1/subdir1/subsubdir2").unwrap();
            let dir = create_dir_all(db, &dir).await;
            db.create_file(&dir, filename1, "file1", 3).unwrap();
            db.create_file(&dir, filename2, "file2", 4).unwrap();
        }
        {
            let dir = DirectoryPathBuf::from_str("dir1/subdir1").unwrap();
            let dir = db.get_directory(&dir).unwrap();
            db.create_file(&dir, filename1, "file1", 5).unwrap();
        }
        {
            let dir = DirectoryPathBuf::from_str("dir1/subdir2").unwrap();
            let dir = create_dir_all(db, &dir).await;
            db.create_file(&dir, filename1, "file1", 6).unwrap();
            db.create_file(&dir, filename2, "file2", 7).unwrap();
        }
        {
            let dir = DirectoryPathBuf::from_str("dir2/subdir1").unwrap();
            let _ = create_dir_all(db, &dir).await;
        }
    }

    #[tokio::test]
    async fn test_walk_directory() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;
        populate_db(&mut db).await;

        // No filters
        {
            let (tx, rx) = flume::bounded(100);
            db.walk_directory(tx, IgnoreFilter::empty("")).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();

            assert_eq!(
                all_files_str,
                vec![
                    "dir1/subdir1/file1.txt",
                    "dir1/subdir1/subsubdir1/file1.txt",
                    "dir1/subdir1/subsubdir1/file2.txt",
                    "dir1/subdir1/subsubdir2/file1.txt",
                    "dir1/subdir1/subsubdir2/file2.txt",
                    "dir1/subdir2/file1.txt",
                    "dir1/subdir2/file2.txt"
                ]
            );
        }

        // Filter all txt files
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["*.txt"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            db.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();
            assert!(all_files_str.is_empty());
        }

        // Filter '*subdir2/' folders
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["*subdir2/"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            db.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();

            assert_eq!(
                all_files_str,
                vec![
                    "dir1/subdir1/file1.txt",
                    "dir1/subdir1/subsubdir1/file1.txt",
                    "dir1/subdir1/subsubdir1/file2.txt"
                ]
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_metadata() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;
        populate_db(&mut db).await;

        // File found
        let dir = DirectoryPathBuf::from_str("dir1/subdir1/subsubdir1").unwrap();
        let metadata = db
            .get_metadata(&dir.join_filename(FilenameBuf::from_str("file1.txt").unwrap()))
            .await?;
        assert_eq!(metadata.path().as_str(), "dir1/subdir1/subsubdir1/file1.txt");
        assert_eq!(metadata.hash(), "file1");
        assert_eq!(metadata.size(), 1);

        // Directory doesn't exist
        let dir_not_exist = DirectoryPathBuf::from_str("not/exists").unwrap();
        let r = db
            .get_metadata(&dir_not_exist.join_filename(FilenameBuf::from_str("whatever").unwrap()))
            .await;
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e, Error::PathDoesNotExist), "Assert failed. Error was: {}", e);

        // File is not found in directory
        let r = db
            .get_metadata(&dir.join_filename(FilenameBuf::from_str("file_not_found").unwrap()))
            .await;
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e, Error::PathDoesNotExist), "Assert failed. Error was: {}", e);
        Ok(())
    }

    #[tokio::test]
    async fn test_exists() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;
        populate_db(&mut db).await;

        let dir = DirectoryPathBuf::from_str("dir1/subdir1/subsubdir1").unwrap();
        let found = db
            .exists(&dir.join_filename(FilenameBuf::from_str("file1.txt").unwrap()))
            .await?;
        assert!(found);

        let dir = DirectoryPathBuf::from_str("dir1").unwrap();
        let not_found = db
            .exists(&dir.join_filename(FilenameBuf::from_str("not-found").unwrap()))
            .await?;
        assert!(!not_found);
        Ok(())
    }

    #[tokio::test]
    async fn test_open() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let db = DatabaseImpl::new(db_file.path().as_str())?;
        let root = DirectoryPathBuf::root();
        let r = db
            .open(&root.join_filename(FilenameBuf::from_str("anything").unwrap()))
            .await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden));
        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let db = DatabaseImpl::new(db_file.path().as_str())?;
        let root = DirectoryPathBuf::root();
        let r = db
            .open(&root.join_filename(FilenameBuf::from_str("anything").unwrap()))
            .await;
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_dir_all() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;
        let long_dir = DirectoryPathBuf::from_str("a/long/dir").unwrap();
        db.create_dir_all(&long_dir).await?;

        let _ = db.get_directory(&DirectoryPathBuf::from_str("a").unwrap())?;
        let _ = db.get_directory(&DirectoryPathBuf::from_str("a/long").unwrap())?;
        let _ = db.get_directory(&DirectoryPathBuf::from_str("a/long/dir").unwrap())?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_file() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;
        let root_dir_path = DirectoryPathBuf::root();
        let filename = FilenameBuf::from_str("file1.txt").unwrap();
        let filepath = root_dir_path.join_filename(&filename);
        {
            assert!(!db.exists(&filepath).await?);
            let root_dir = db.get_directory(&root_dir_path)?;
            db.create_file(&root_dir, &filename, "file1", 1)?;
            assert!(db.exists(&filepath).await?);
        }
        db.remove_file(&filepath).await?;
        assert!(!db.exists(&filepath).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_dir() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;

        // A directory without files
        let dir = DirectoryPathBuf::from_str("a/long/dir").unwrap();
        db.create_dir_all(&dir).await?;
        let r = db.remove_dir(&dir).await;
        assert!(r.is_ok());
        assert!(db.get_directory(&dir).is_err());
        assert!(db.get_directory(dir.parent().unwrap()).is_ok());

        // A directory with files
        let dir_object = create_dir_all(&mut db, &dir).await;
        db.create_file(&dir_object, &FilenameBuf::from_str("file.txt").unwrap(), "file1", 1)?;
        let r = db.remove_dir(&dir).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::NotEmptyDirectory),
            "Assert failed. Error was: '{}'",
            e
        );

        // A directory with child directories
        db.create_dir_all(&dir).await?;
        let r = db.remove_dir(dir.parent().unwrap()).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::NotEmptyDirectory),
            "Assert failed. Error was: '{}'",
            e
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_dir_all() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;

        // A directory without files
        let dir = DirectoryPathBuf::from_str("a/long/dir").unwrap();
        db.create_dir_all(&dir).await?;
        db.remove_dir_all(&dir).await?;
        assert!(db.get_directory(&dir).is_err());
        assert!(db.get_directory(dir.parent().unwrap()).is_ok());

        // A directory with files
        let dir_object = create_dir_all(&mut db, &dir).await;
        let filename = FilenameBuf::from_str("file.txt").unwrap();
        db.create_file(&dir_object, &filename, "file1", 1)?;
        db.remove_dir_all(&dir).await?;
        assert!(db.get_directory(&dir).is_err());
        assert!(!db.exists(&dir.join_filename(&filename)).await?);
        assert!(db.get_directory(dir.parent().unwrap()).is_ok());

        // A directory with child directories
        db.create_dir_all(&dir).await?;
        db.remove_dir_all(dir.parent().unwrap().parent().unwrap()).await?;
        assert!(db.get_directory(&dir).is_err());
        assert!(db.get_directory(dir.parent().unwrap()).is_err());
        assert!(db.get_directory(dir.parent().unwrap().parent().unwrap()).is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_internal_copy() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;

        let dir = DirectoryPathBuf::from_str("a/long/dir").unwrap();
        let dir_object = create_dir_all(&mut db, &dir).await;
        db.create_file(&dir_object, &FilenameBuf::from_str("file.txt").unwrap(), "file1", 1)?;
        db.create_file(&dir_object, &FilenameBuf::from_str("file2.txt").unwrap(), "file2", 32)?;

        let file_txt = dir.join_filename(FilenameBuf::from_str("file.txt").unwrap());
        let file2_txt = dir.join_filename(FilenameBuf::from_str("file2.txt").unwrap());
        let file_copy_txt = dir.join_filename(FilenameBuf::from_str("file_copy.txt").unwrap());

        // Target directory doesn't exist
        let r = db
            .internal_copy(
                &file_txt,
                &FilePathBuf::new(
                    DirectoryPathBuf::from_str("another").unwrap(),
                    FilenameBuf::from_str("place.txt").unwrap(),
                ),
                true,
            )
            .await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::PathDoesNotExist),
            "Assert failed. Error was: '{}'",
            e
        );

        // Target file already exist (force=false)
        let r = db.internal_copy(&file_txt, &file2_txt, false).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::TargetFileExists),
            "Assert failed. Error was: '{}'",
            e
        );

        // Target file doesn't exist
        assert!(!db.exists(&file_copy_txt).await?);
        let r = db.internal_copy(&file_txt, &file_copy_txt, false).await?;
        assert!(r.is_none()); // Nothing to wait
        assert!(db.exists(&file_copy_txt).await?);

        // Target file is overridden
        let r = db.internal_copy(&file_txt, &file2_txt, true).await?;
        assert!(r.is_none()); // Nothing to wait
        let metadata = db.get_metadata(&file2_txt).await?;
        assert_eq!(metadata.hash(), "file1");
        assert_eq!(metadata.size(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_internal_move() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;

        let dir = DirectoryPathBuf::from_str("a/long/dir").unwrap();
        let dir_object = create_dir_all(&mut db, &dir).await;
        db.create_file(&dir_object, &FilenameBuf::from_str("file.txt").unwrap(), "file1", 1)?;
        db.create_file(&dir_object, &FilenameBuf::from_str("file2.txt").unwrap(), "file2", 32)?;

        let file_txt = dir.join_filename(FilenameBuf::from_str("file.txt").unwrap());
        let file2_txt = dir.join_filename(FilenameBuf::from_str("file2.txt").unwrap());
        let file_copy_txt = dir.join_filename(FilenameBuf::from_str("file_copy.txt").unwrap());

        // Target directory doesn't exist
        let r = db
            .internal_move(
                &file_txt,
                &FilePathBuf::new(
                    DirectoryPathBuf::from_str("another").unwrap(),
                    FilenameBuf::from_str("place.txt").unwrap(),
                ),
                true,
            )
            .await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::PathDoesNotExist),
            "Assert failed. Error was: '{}'",
            e
        );

        // Target file already exist (force=false)
        let r = db.internal_move(&file_txt, &file2_txt, false).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::TargetFileExists),
            "Assert failed. Error was: '{}'",
            e
        );

        // Target file doesn't exist
        assert!(!db.exists(&file_copy_txt).await?);
        let r = db.internal_move(&file_txt, &file_copy_txt, false).await?;
        assert!(r.is_none()); // Nothing to wait
        assert!(db.exists(&file_copy_txt).await?);
        assert!(!db.exists(&file_txt).await?);

        // Target file is overridden
        let r = db.internal_move(&file_copy_txt, &file2_txt, true).await?;
        assert!(r.is_none()); // Nothing to wait
        let metadata = db.get_metadata(&file2_txt).await?;
        assert_eq!(metadata.hash(), "file1");
        assert_eq!(metadata.size(), 1);
        assert!(!db.exists(&file_copy_txt).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_copy_from() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;

        // // If it's within the DB filesystem, it's just a `copy_internal`:
        // TODO: I can't get mutable and inmutable borrow at the same time
        // db.copy_from(Utf8Path::new("copy.txt"), &db, Utf8Path::new("file.txt"), true).await?;
        // assert!(db.exists(Utf8Path::new("copy.txt")).await?);

        let file_txt = DirectoryPathBuf::root().join_filename(FilenameBuf::from_str("file.txt").unwrap());
        let copy_txt = DirectoryPathBuf::root().join_filename(FilenameBuf::from_str("copy.txt").unwrap());

        let mut temp_fs = FilesystemLocalTemp::default();
        let (_, _) = temp_fs.create(&file_txt).await?;

        db.copy_from(&copy_txt, &temp_fs, &file_txt, false).await?;
        assert!(db.exists(&copy_txt).await?);
        assert!(!db.exists(&file_txt).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_move_from() -> anyhow::Result<()> {
        let db_file = NamedUtf8TempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().as_str())?;

        // // If it's within the DB filesystem, it's just a `copy_internal`:
        // TODO: I can't get mutable and inmutable borrow at the same time
        // db.move_from(Utf8Path::new("copy.txt"), &db, Utf8Path::new("file.txt"), true).await?;
        // assert!(db.exists(Utf8Path::new("copy.txt")).await?);

        let file_txt = DirectoryPathBuf::root().join_filename(FilenameBuf::from_str("file.txt").unwrap());
        let copy_txt = DirectoryPathBuf::root().join_filename(FilenameBuf::from_str("copy.txt").unwrap());

        let mut temp_fs = FilesystemLocalTemp::default();
        let (_, _) = temp_fs.create(&file_txt).await?;

        let r = db.move_from(&copy_txt, &mut temp_fs, &file_txt, false).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden), "Assert failed. Error was: {}", e);
        assert!(!db.exists(&copy_txt).await?);
        assert!(temp_fs.exists(&file_txt).await?);
        Ok(())
    }

    #[test]
    fn test_get_or_create_directory_all() {
        let db_file = NamedUtf8TempFile::new().unwrap();
        let db = DatabaseImpl::new(db_file.path().as_str()).unwrap();

        {
            let path = DirectoryPathBuf::from_str("a/path/to/something").unwrap();
            let (dir, created) = db.get_or_create_directory_all(&path).unwrap();
            assert!(created);
            assert_eq!(dir.full_path().as_str(), "a/path/to/something");

            // all the parents have been created
            let parent_dir = db.get_directory(path.parent().unwrap()).unwrap();
            assert_eq!(parent_dir.full_path().as_str(), "a/path/to");

            let parent_dir = db.get_directory(parent_dir.full_path().parent().unwrap()).unwrap();
            assert_eq!(parent_dir.full_path().as_str(), "a/path");

            let parent_dir = db.get_directory(parent_dir.full_path().parent().unwrap()).unwrap();
            assert_eq!(parent_dir.full_path().as_str(), "a");
        }

        {
            let path = DirectoryPathBuf::from_str("a/path/to").unwrap();
            let (dir, created) = db.get_or_create_directory_all(&path).unwrap();
            assert!(!created);
            assert_eq!(dir.full_path().as_str(), "a/path/to");
        }

        {
            let path = DirectoryPathBuf::from_str("a/path/to/something_else").unwrap();
            let (dir, created) = db.get_or_create_directory_all(&path).unwrap();
            assert!(created);
            assert_eq!(dir.full_path().as_str(), "a/path/to/something_else");
        }
    }
}
