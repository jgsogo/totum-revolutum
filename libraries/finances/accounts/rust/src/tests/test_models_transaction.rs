use crate::models::{NewTransaction, Transaction};
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    // All transactions
    {
        let all_transactions = Transaction::all()
            .select(Transaction::as_select())
            .load::<Transaction>(&mut conn)
            .expect("Error counting 'all' transactions");
        assert_eq!(all_transactions.len(), 3);
    }

    // Add a transaction
    let new_transaction = NewTransaction {
        name: "New transaction",
        description: Some("Some description"),
        group_id: None,
    };

    let r = diesel::insert_into(crate::schema::finances_accounts_transaction::table)
        .values(&new_transaction)
        .returning(crate::schema::finances_accounts_transaction::id)
        .get_result::<i64>(&mut conn);
    assert!(r.is_ok());
}
