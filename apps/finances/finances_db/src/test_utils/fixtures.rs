use super::SqliteTestDatabase;

// FIXME: Use `rstest` crate to create actual fixtures here (it failed for me in the Bazel build)

/// Returns a database with hardcoded [`MovementType`] and [`AccountType`]
pub fn database() -> SqliteTestDatabase {
    let mut database = SqliteTestDatabase::new();
    database
        .populate_movement_types()
        .expect("Error populating movement types");
    database
        .populate_account_types()
        .expect("Error populating account types");
    database
}

/// Returns a database with hardcoded [`MovementType`] and [`AccountType`], and some [`Account`]s
/// and [`AccountHolder`]s
pub fn database_with_accounts() -> SqliteTestDatabase {
    let mut database = database();
    database
        .populate_account_holders()
        .expect("Error populating account types");
    database.populate_accounts().expect("Error populating account types");
    database
}
