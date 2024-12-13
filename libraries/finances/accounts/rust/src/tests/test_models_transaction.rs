use super::DB_POOL;
use crate::models::{NewTransaction, Transaction};
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

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

    let r = new_transaction.insert_into_db(&mut conn);
    assert!(r.is_ok());
}
