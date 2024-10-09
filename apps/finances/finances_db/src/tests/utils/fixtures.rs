use super::SqliteTestDatabase;

// FIXME: Use `rstest` crate to create actual fixtures here (it failed for me in the Bazel build)

pub fn database_with_accounts() -> SqliteTestDatabase {
    let mut database = SqliteTestDatabase::new();
    database
        .populate_account_types()
        .expect("Error populating account types");
    database
        .populate_account_holders()
        .expect("Error populating account types");
    database.populate_accounts().expect("Error populating account types");
    database
}
