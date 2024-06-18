//! Default implementation for [`super::FilesystemIndexedDatabase`] using a database implemented
//! with the `diesel` crate.

use database::DatabaseImpl;

use crate::impls::composites::FilesystemIndexed;
use crate::{Filesystem, Result};

mod database;
mod models;
mod schema;

/// Creates a new [`FilesystemIndexed`] using a default implementation of a database (SQLite3)
#[allow(private_interfaces)]
pub async fn new_filesystem_indexed_with_db<TStorage: Filesystem + 'static>(
    database_url: &str,
    storage: TStorage,
    do_initial_indexing: bool,
) -> Result<FilesystemIndexed<DatabaseImpl, TStorage>> {
    let database = DatabaseImpl::new(database_url)?;
    let fs_indexed = FilesystemIndexed::new(database, storage);
    if do_initial_indexing {
        fs_indexed.sync().await?;
    }
    Ok(fs_indexed)
}

#[cfg(test)]
mod tests {
    use camino::{Utf8Path, Utf8PathBuf};

    use crate::impls::composites::indexed::diesel_indexed::database::DatabaseImpl;
    use crate::impls::composites::{new_filesystem_indexed_with_db, FilesystemIndexedDatabase};
    use crate::impls::FilesystemLocalTemp;
    use crate::Filesystem;

    #[tokio::test]
    async fn test_filesystem_indexed_db() -> anyhow::Result<()> {
        let database_file = tempfile::NamedTempFile::new()?;
        let mut fs = {
            let fs = FilesystemLocalTemp::default();
            new_filesystem_indexed_with_db(database_file.path().to_str().unwrap(), fs, false).await?
        };

        // Populate the filesystem with some files and directories
        {
            // - a file in the root folder
            let rx = {
                let (mut f, rx) = fs.create(Utf8Path::new("file.txt")).await?;
                f.write_all(b"Some content in the root").await?;
                f.sync_all().await?;
                rx.unwrap()
            };
            let _ = rx.await.unwrap();

            // - a file inside some folder
            let rx = {
                fs.create_dir_all(Utf8Path::new("a/folder")).await?;
                let (mut f, rx) = fs.create(Utf8Path::new("a/folder/file.txt")).await?;
                f.write_all(b"Some other content").await?;
                f.sync_all().await?;
                rx.unwrap()
            };
            let _ = rx.await.unwrap();
        }

        // The filesystem tell us about the files available
        let (tx, rx) = flume::bounded(10);
        fs.walk_directory(tx, 10, Utf8Path::new("")).await?;
        let all_files = rx.try_iter().collect::<Vec<_>>();
        assert_eq!(all_files.len(), 2);

        // Now we can go directly to the database and check it
        let db = DatabaseImpl::new(database_file.path().to_str().unwrap())?;
        let files = db.all_files()?.collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let f0 = files.get(0).unwrap();
        let f1 = files.get(1).unwrap();

        let mut conn = db.get_conn()?;
        assert_eq!(f0.full_path(&mut conn)?, Utf8PathBuf::from("file.txt"));
        assert_eq!(f1.full_path(&mut conn)?, Utf8PathBuf::from("a/folder/file.txt"));

        Ok(())
    }
}
