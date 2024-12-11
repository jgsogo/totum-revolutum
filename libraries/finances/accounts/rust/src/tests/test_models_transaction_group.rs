use crate::models::TransactionGroup;
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let all_transaction_groups = TransactionGroup::all()
        .select(TransactionGroup::as_select())
        .load::<TransactionGroup>(&mut conn)
        .expect("Error counting 'all' transaction groups");
    assert_eq!(all_transaction_groups.len(), 2);

    assert_eq!(all_transaction_groups.get(0).unwrap().name, "transaction_group0");
    assert_eq!(all_transaction_groups.get(1).unwrap().name, "transaction_group1");
}
