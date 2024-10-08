use super::TestDatabase;
use anyhow::{anyhow, Result};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tempfile::NamedTempFile;
use tracing::debug;

// FIXME: Using rstest to create actual fixtures here would be great, but I'm finding some issues trying to use that crate using Bazel.

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    // Create the SQLite3 database and run migrations
    debug!("Run pending migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("Error {}", e))?;
    Ok(())
}

pub fn database() -> TestDatabase {
    let dbfile = NamedTempFile::new().expect("Failed to create temporary file");
    let dbfile_str = dbfile.path().to_str().unwrap();
    let mut conn = SqliteConnection::establish(dbfile_str).expect("Failed to establish connection to database");
    diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
        .execute(&mut conn)
        .unwrap();

    run_migrations(&mut conn).expect("Failed to run migrations");

    TestDatabase::new(dbfile, conn)
}

pub fn database_with_accounts() -> TestDatabase {
    let mut database = database();
    database
        .populate_account_types()
        .expect("Error populating account types");
    database
        .populate_account_holders()
        .expect("Error populating account types");
    database.populate_accounts().expect("Error populating account types");
    database
}
