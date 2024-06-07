///! Helpers to create a connection to an in-memory database (SQLite). Source code taken from
/// `diesel_tests` crate at https://github.com/diesel-rs/diesel/blob/master/diesel_tests/tests/schema/mod.rs
use diesel::*;
#[cfg(feature = "sqlite")]
pub type TestConnection = SqliteConnection;

#[cfg(feature = "sqlite")]
const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("tests/common/migrations");

pub fn connection() -> TestConnection {
    let mut result = connection_without_transaction();
    result.begin_test_transaction().unwrap();
    result
}

pub fn connection_without_transaction() -> TestConnection {
    use diesel_migrations::MigrationHarness;

    let mut result = backend_specific_connection();

    if cfg!(feature = "sqlite") {
        result.run_pending_migrations(MIGRATIONS).unwrap();
    } else {
        todo!("not impl")
    }

    result
}

#[cfg(feature = "sqlite")]
pub fn backend_specific_connection() -> TestConnection {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
        .execute(&mut conn)
        .unwrap();
    conn
}
