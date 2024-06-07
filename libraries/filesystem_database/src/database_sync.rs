/// Implements [`filesystem::Filesystem`] for a database that contains references to files in
/// another _filesystem_. With this implementation the content of the files can't be retrieved
/// which limits the features that are available.
///
/// Use this implementation to sync the database with the filesystem it is related to: add missing
/// files and remove entries from the DB for files that have been remove.
///
/// See [`super::DatabaseBackup`] implementation if you are looking for an implementation that
/// can access the actual files.
pub struct DatabaseSync {}
