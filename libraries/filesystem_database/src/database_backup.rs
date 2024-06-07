/// Implements [`filesystem::Filesystem`] for a _filesystem_ whose content is proxied in a
/// database. The database is the source of truth for the filesystem content, although the content
/// of the files is retrieved from the actual _filesystem_.
///
/// Note that the database and the storage may get out-of-sync if some files are added/removed to
/// the storage without updating the database (see [`super::DatabaseSync`]).
pub struct DatabaseBackup {}
