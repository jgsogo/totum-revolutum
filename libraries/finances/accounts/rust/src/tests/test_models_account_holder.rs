use super::DB_POOL;
use crate::models::{Account, AccountHolder};
use crate::sql::filters::{account_by_pk, accountholder_by_pk};
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

    {
        let all: i64 = AccountHolder::all()
            .count()
            .get_result(&mut conn)
            .expect("Error counting 'all' holders");
        assert_eq!(all, 3);
    }

    {
        // Holders for a given account
        let account_id = 0i64;
        let holders = Account::all()
            .inner_join(
                crate::schema::finances_accounts_accountholderrole::table
                    .inner_join(crate::schema::finances_accounts_accountholder::table),
            )
            .filter(account_by_pk(account_id))
            .select((
                AccountHolder::as_select(),
                crate::schema::finances_accounts_accountholderrole::owns_money,
            ))
            .load::<(AccountHolder, bool)>(&mut conn)
            .expect("Error retrieving holders for a given account");
        assert_eq!(holders.len(), 3);
        {
            let (holder, owns_money) = holders.get(0).unwrap();
            assert_eq!(holder.name, "holder0");
            assert!(owns_money);
        }
        {
            let (holder, owns_money) = holders.get(1).unwrap();
            assert_eq!(holder.name, "holder1");
            assert!(owns_money);
        }
        {
            let (holder, owns_money) = holders.get(2).unwrap();
            assert_eq!(holder.name, "holder2");
            assert!(!owns_money);
        }
    }

    {
        // Acconts (only opened ones) for a given holder
        let holder_id = 1i64;
        let accounts = Account::all_opened()
            .inner_join(
                crate::schema::finances_accounts_accountholderrole::table
                    .inner_join(crate::schema::finances_accounts_accountholder::table),
            )
            .filter(accountholder_by_pk(holder_id))
            .select((
                Account::as_select(),
                crate::schema::finances_accounts_accountholderrole::owns_money,
            ))
            .load::<(Account, bool)>(&mut conn)
            .expect("Error retrieving accounts for a given holder");

        assert_eq!(accounts.len(), 2);

        {
            let (account, owns_money) = accounts.get(0).unwrap();
            assert_eq!(account.name, "Gastos compartidos");
            assert!(owns_money);
        }
        {
            let (account, owns_money) = accounts.get(1).unwrap();
            assert_eq!(account.name, "Depósito 3M");
            assert!(owns_money);
        }

        // ... I can also query closed accounts
        let closed_accounts = Account::all_closed()
            .inner_join(
                crate::schema::finances_accounts_accountholderrole::table
                    .inner_join(crate::schema::finances_accounts_accountholder::table),
            )
            .filter(accountholder_by_pk(holder_id))
            .select((
                Account::as_select(),
                crate::schema::finances_accounts_accountholderrole::owns_money,
            ))
            .load::<(Account, bool)>(&mut conn)
            .expect("Error retrieving accounts for a given holder");
        assert_eq!(closed_accounts.len(), 1);

        {
            let (account, owns_money) = closed_accounts.get(0).unwrap();
            assert_eq!(account.name, "Old account");
            assert!(owns_money);
        }
    }
}
