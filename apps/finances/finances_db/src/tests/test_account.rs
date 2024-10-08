use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::{Account, AccountHolder, AccountType};
use crate::tests::utils::fixtures::database_with_accounts;

#[test]
fn test_all_with_holder_and_type() {
    let mut database_with_accounts = database_with_accounts();
    let all = Account::all_with_holder_and_type()
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
        .expect("Error loading accounts");

    assert_eq!(all.len(), 3);
}
