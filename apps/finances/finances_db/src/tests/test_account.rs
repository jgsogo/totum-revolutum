use crate::models::{Account, AccountHolder, AccountType};
use crate::tests::utils::fixtures::database_with_accounts;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

#[test]
fn test_all_with_holder_and_type() {
    let mut database_with_accounts = database_with_accounts();
    let all = QueryDsl::order(
        Account::all_with_holder_and_type(),
        crate::schema::data_account::open.desc(),
    )
    .select((
        Account::as_select(),
        AccountHolder::as_select(),
        AccountType::as_select(),
    ))
    .load::<(Account, AccountHolder, AccountType)>(&mut database_with_accounts.conn)
    .expect("Error loading accounts");

    assert_eq!(all.len(), 3);

    {
        let (acc, holder, acc_type) = all.get(0).unwrap();
        assert_eq!(acc.name, "Netflix");
        assert_eq!(holder.name, "holder2");
        assert_eq!(acc_type.name, "Acciones");
    }
    {
        let (acc, holder, acc_type) = all.get(1).unwrap();
        assert_eq!(acc.name, "IBM");
        assert_eq!(holder.name, "holder1");
        assert_eq!(acc_type.name, "Acciones");
    }
    {
        let (acc, holder, acc_type) = all.get(2).unwrap();
        assert_eq!(acc.name, "Gastos compartidos");
        assert_eq!(holder.name, "holder0");
        assert_eq!(acc_type.name, "Cuenta Corriente");
    }
}
