use super::DB_POOL;
use crate::models::TransactionGroup;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

    let all_transaction_groups = TransactionGroup::all()
        .select(TransactionGroup::as_select())
        .load::<TransactionGroup>(&mut conn)
        .expect("Error counting 'all' transaction groups");
    assert_eq!(all_transaction_groups.len(), 2);

    assert_eq!(all_transaction_groups.get(0).unwrap().name, "transaction_group0");
    assert_eq!(all_transaction_groups.get(1).unwrap().name, "transaction_group1");
}
