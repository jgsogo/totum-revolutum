use crate::models::{Account, AccountType, Custodian};
use crate::query_fragments::{account_closed, account_opened, account_ordered, custodian_by_pk};
use crate::test_utils::fixtures::database_with_accounts;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut database_with_accounts = database_with_accounts();

    {
        let all: i64 = Account::all()
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error counting 'all' accounts");
        assert_eq!(all, 2); // Only the opened accounts are returned
        let opened: i64 = Account::all()
            .filter(account_opened())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error counting 'opened' accounts");
        assert_eq!(opened, 2);
        let closed: i64 = Account::all()
            .filter(account_closed())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error counting 'closed' accounts");
        assert_eq!(closed, 0); // 'all' returns only the opened ones, then we filter for the closed ones :D
        let all_closed: i64 = Account::all_closed()
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error counting 'all_closed' accounts");
        assert_eq!(all_closed, 1); // 'all_closed' returns only the closed ones
    }

    // All accounts (opened ones)
    {
        let all = Account::all()
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .order(crate::schema::finances_accounts_account::open.desc())
            .load::<(Account, Custodian, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");

        assert_eq!(all.len(), 2);

        {
            let (acc, custodian, acc_type) = all.get(0).unwrap();
            assert_eq!(acc.name, "IBM");
            assert_eq!(custodian.name, "custodian1");
            assert_eq!(
                acc_type.unique_name.as_ref().unwrap(),
                crate::constants::accounttype::ASSETS
            );
        }
        {
            let (acc, custodian, acc_type) = all.get(1).unwrap();
            assert_eq!(acc.name, "Gastos compartidos");
            assert_eq!(custodian.name, "custodian0");
            assert_eq!(
                acc_type.unique_name.as_ref().unwrap(),
                crate::constants::accounttype::ASSETS_CURRENT
            );
        }
    }

    // Closed accounts
    {
        let all = Account::all_closed()
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .order(account_ordered())
            .load::<(Account, Custodian, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");

        assert_eq!(all.len(), 1);

        {
            let (acc, custodian, acc_type) = all.get(0).unwrap();
            assert_eq!(acc.name, "Netflix");
            assert_eq!(custodian.name, "custodian2");
            assert_eq!(
                acc_type.unique_name.as_ref().unwrap(),
                crate::constants::accounttype::ASSETS
            );
        }
    }

    // Get all accounts for a given Custodian
    {
        let accounts = Account::all()
            .filter(custodian_by_pk(0))
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .load::<(Account, Custodian, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 1);

        let (acc, custodian, acc_type) = accounts.get(0).unwrap();
        assert_eq!(acc.name, "Gastos compartidos");
        assert_eq!(custodian.name, "custodian0");
        assert_eq!(
            acc_type.unique_name.as_ref().unwrap(),
            crate::constants::accounttype::ASSETS_CURRENT
        );
    }
}
