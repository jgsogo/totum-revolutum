use super::DB_POOL;
use crate::models::{NewSnapshotNumerable, SnapshotNumerable};
use crate::sql::queries::all_snapshotnumerable_for_account_id;
use diesel::prelude::*;
use finances_accounts::models::NewSnapshot;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

    let account_id = 4;

    // All snapshots (as SnapshotNumerable) for a given account
    {
        let all: Vec<SnapshotNumerable> = all_snapshotnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(account_id)
            .load(&mut conn)
            .expect("Error loading snapshots");
        assert_eq!(all.len(), 2);
    }

    // Insert one more snapshot numerable
    let amount: bigdecimal::BigDecimal = 3f32.try_into().unwrap();
    let date_value = chrono::NaiveDate::parse_from_str("2024-09-10", "%Y-%m-%d").unwrap();
    let quantity: bigdecimal::BigDecimal = 5f32.try_into().unwrap();
    let unit_value: bigdecimal::BigDecimal = 500f32.try_into().unwrap();
    let new_snapshot_numerable = NewSnapshotNumerable {
        new_snapshot: &NewSnapshot {
            account_id: &account_id,
            amount: &amount,
            date_value: &date_value,
        },
        quantity: &quantity,
        unit_value: &unit_value,
    };
    new_snapshot_numerable.insert_into_db(&mut conn).unwrap();

    let all: Vec<SnapshotNumerable> = all_snapshotnumerable_for_account_id()
        .bind::<diesel::sql_types::Int8, _>(account_id)
        .load(&mut conn)
        .expect("Error loading snapshots");
    assert_eq!(all.len(), 3);
}
