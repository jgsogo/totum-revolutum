//! Default implementation for [`super::FilesystemIndexedDatabase`] using a database implemented
//! with the `diesel` crate.

use database::DatabaseImpl;

use crate::impls::composites::FilesystemIndexed;
use crate::{Filesystem, Result};

mod database;
mod models;
mod schema;

/// Creates a new [`FilesystemIndexed`] using a default implementation of a database (SQLite3)
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

    #[tokio::test]
    async fn test_filesystem_indexed_db() {
        // TODO: Implement tests for this 'new_filesystem_indexed_with_db'!
    }
}
