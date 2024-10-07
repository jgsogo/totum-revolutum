//! These tests should guarantee that the source code we write (and exercise in production using
//! the production database) works with the migrations and the auto-generated schema.

use anyhow::{anyhow, Result};
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tempfile::NamedTempFile;
use tracing::debug;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    // Create the SQLite3 database and run migrations
    debug!("Run pending migrations");
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("Error {}", e))?;
    Ok(())
}

#[test]
fn test_formats_are_propulated() -> Result<()> {
    let dbfile = NamedTempFile::new()?;
    let dbfile_str = dbfile.path().to_str().unwrap();
    let mut conn = SqliteConnection::establish(dbfile_str)?;
    diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
        .execute(&mut conn)
        .unwrap();
    run_migrations(&mut conn)?;

    // let mut conn = SqliteConnection::establish(dbfile_str)?;
    // for it in models::Formats::iter() {
    //     let f = Format::find(&it, &mut conn)?;
    //     let formats_: models::Formats = f.into();
    //     assert_eq!(formats_, it);
    // }

    Ok(())
}
