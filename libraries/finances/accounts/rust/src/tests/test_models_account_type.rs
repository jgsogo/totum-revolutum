use crate::models::AccountType;
use crate::test_utils::TestDatabase;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut database = TestDatabase::new();

    // All account types
    {
        let all = crate::schema::finances_accounts_accounttype::table
            .select(AccountType::as_select())
            .load::<AccountType>(&mut database.conn)
            .expect("Error loading account types");

        assert_eq!(all.len(), 14);
    }
}
