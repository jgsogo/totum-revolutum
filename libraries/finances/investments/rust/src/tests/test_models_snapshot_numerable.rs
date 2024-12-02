use crate::managers::create_snapshot_numerable;
use crate::models::SnapshotNumerable;
use crate::sql::queries::all_snapshotnumerable_for_account_id;
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

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
    create_snapshot_numerable(&mut conn, &account_id, &amount, &date_value, &quantity, &unit_value).unwrap();

    let all: Vec<SnapshotNumerable> = all_snapshotnumerable_for_account_id()
        .bind::<diesel::sql_types::Int8, _>(account_id)
        .load(&mut conn)
        .expect("Error loading snapshots");
    assert_eq!(all.len(), 3);
}
