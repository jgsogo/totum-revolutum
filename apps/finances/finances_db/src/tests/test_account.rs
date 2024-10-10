use crate::models::account_type::{ACCIONES, CUENTA_CORRIENTE};
use crate::models::{Account, AccountHolder, AccountType};
use crate::test_utils::fixtures::database_with_accounts;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut database_with_accounts = database_with_accounts();

    // All accounts
    {
        let all = Account::all_with_holder_and_type()
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .order(crate::schema::data_account::open.desc())
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");

        assert_eq!(all.len(), 3);

        {
            let (acc, holder, acc_type) = all.get(0).unwrap();
            assert_eq!(acc.name, "Netflix");
            assert_eq!(holder.name, "holder2");
            assert_eq!(acc_type.name, ACCIONES);
        }
        {
            let (acc, holder, acc_type) = all.get(1).unwrap();
            assert_eq!(acc.name, "IBM");
            assert_eq!(holder.name, "holder1");
            assert_eq!(acc_type.name, ACCIONES);
        }
        {
            let (acc, holder, acc_type) = all.get(2).unwrap();
            assert_eq!(acc.name, "Gastos compartidos");
            assert_eq!(holder.name, "holder0");
            assert_eq!(acc_type.name, CUENTA_CORRIENTE);
        }
    }

    // Get account by pk
    {
        let accounts = Account::get_with_holder_and_type(0)
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 1);

        let (acc, holder, acc_type) = accounts.get(0).unwrap();
        assert_eq!(acc.name, "Gastos compartidos");
        assert_eq!(holder.name, "holder0");
        assert_eq!(acc_type.name, CUENTA_CORRIENTE);
    }
}

#[test]
fn test_filters() {
    let mut database_with_accounts = database_with_accounts();

    // opened
    {
        let accounts = Account::all_with_holder_and_type()
            .filter(Account::opened())
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 2);
    }

    // mine
    {
        let accounts = Account::all_with_holder_and_type()
            .filter(Account::mine())
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 3);
    }

    // checking accounts
    {
        let accounts = Account::all_with_holder_and_type()
            .filter(Account::checking_account())
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 1);
    }

    // Investments
    {
        let accounts = Account::all_with_holder_and_type()
            .filter(Account::investment())
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 2);
    }

    // retirement
    {
        let accounts = Account::all_with_holder_and_type()
            .filter(Account::retirement())
            .select((
                Account::as_select(),
                AccountHolder::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 0);
    }
}
