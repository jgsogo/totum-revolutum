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
        let accounts: i64 = Account::all_with_holder_and_type()
            .filter(Account::opened())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts, 2);
    }

    // mine
    {
        let accounts: i64 = Account::all_with_holder_and_type()
            .filter(Account::mine())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts, 3);
    }

    // checking accounts
    {
        let accounts: i64 = Account::all_with_holder_and_type()
            .filter(Account::checking_account())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts, 1);
    }

    // Investments
    {
        let accounts: i64 = Account::all_with_holder_and_type()
            .filter(Account::investment())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts, 2);
    }

    // retirement
    {
        let accounts: i64 = Account::all_with_holder_and_type()
            .filter(Account::retirement())
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading accounts");
        assert_eq!(accounts, 0);
    }
}

#[test]
fn test_constraints() {
    let mut database_with_accounts = database_with_accounts();

    // we start with 3 accounts
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

    // I cannot remove a account-type if there are accounts using it
    {
        let (_, _, acc_type) = all.get(0).unwrap();
        let r = diesel::delete(
            crate::schema::data_accounttype::dsl::data_accounttype
                .filter(crate::schema::data_accounttype::id.eq(acc_type.id)),
        )
        .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::Unknown));
                assert_eq!(info.message(), "FOREIGN KEY constraint failed");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // If I remove an account holder, I remove all associated accounts
    {
        let (_, holder, _) = all.get(0).unwrap();
        let r = diesel::delete(
            crate::schema::data_accountholder::dsl::data_accountholder
                .filter(crate::schema::data_accountholder::id.eq(holder.id)),
        )
        .execute(&mut database_with_accounts.conn);

        assert!(r.is_ok());

        let all: i64 = Account::all_with_holder_and_type()
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading accounts");

        assert_eq!(all, 2);
    }
}
