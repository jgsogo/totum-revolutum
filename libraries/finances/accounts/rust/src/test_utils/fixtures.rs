use super::TestDatabase;

// FIXME: Use `rstest` crate to create actual fixtures here (it failed for me in the Bazel build)

/// Returns a database with hardcoded [`MovementType`] and [`AccountType`], and some [`Account`]s
/// and [`Custodian`]s
pub fn database_with_accounts() -> TestDatabase {
    let mut database = TestDatabase::new();
    database.populate_custodians().expect("Error populating account types");
    database.populate_accounts().expect("Error populating accounts");
    database
}
