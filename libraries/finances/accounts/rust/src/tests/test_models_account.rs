use crate::models::{Account, AccountType, Custodian};
use crate::test_utils::fixtures::database_with_accounts;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut database_with_accounts = database_with_accounts();

    // All accounts
    {
        let all = Account::all_with_custodian_and_type()
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .order(crate::schema::finances_accounts_account::open.desc())
            .load::<(Account, Custodian, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");

        assert_eq!(all.len(), 3);

        {
            let (acc, custodian, acc_type) = all.get(0).unwrap();
            assert_eq!(acc.name, "Netflix");
            assert_eq!(custodian.name, "custodian2");
            assert_eq!(
                acc_type.unique_name.as_ref().unwrap(),
                crate::constants::accounttype::ASSETS
            );
        }
        {
            let (acc, custodian, acc_type) = all.get(1).unwrap();
            assert_eq!(acc.name, "IBM");
            assert_eq!(custodian.name, "custodian1");
            assert_eq!(
                acc_type.unique_name.as_ref().unwrap(),
                crate::constants::accounttype::ASSETS
            );
        }
        {
            let (acc, custodian, acc_type) = all.get(2).unwrap();
            assert_eq!(acc.name, "Gastos compartidos");
            assert_eq!(custodian.name, "custodian0");
            assert_eq!(
                acc_type.unique_name.as_ref().unwrap(),
                crate::constants::accounttype::ASSETS_CURRENT
            );
        }
    }

    // Get account by pk
    {
        let accounts = Account::get_with_custodian_and_type(0)
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .load::<(Account, Custodian, AccountType)>(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts.len(), 1);

        let (acc, holder, acc_type) = accounts.get(0).unwrap();
        assert_eq!(acc.name, "Gastos compartidos");
        assert_eq!(holder.name, "custodian0");
        assert_eq!(
            acc_type.unique_name.as_ref().unwrap(),
            crate::constants::accounttype::ASSETS_CURRENT
        );
    }
}
