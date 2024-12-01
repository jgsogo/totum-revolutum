use crate::models::{NewSnapshotNumerable, SnapshotNumerable};
use crate::sql::queries::all_snapshotnumerable_for_account_id;
use crate::test_utils::establish_connection;
use diesel::prelude::*;
use finances_accounts::models::NewSnapshot;

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
    conn.transaction(|conn| {
        let amount: bigdecimal::BigDecimal = 3f32.try_into().unwrap();
        let date_value = chrono::NaiveDate::parse_from_str("2024-09-10", "%Y-%m-%d").unwrap();
        let new_snapshot = NewSnapshot {
            amount: &amount,
            date_value: &date_value,
            account_id: &account_id,
        };
        let inserted = diesel::insert_into(finances_accounts::schema::finances_accounts_snapshot::table)
            .values(&new_snapshot)
            .returning(finances_accounts::schema::finances_accounts_snapshot::id)
            .get_result(conn)?;

        let quantity: bigdecimal::BigDecimal = 5f32.try_into().unwrap();
        let unit_value: bigdecimal::BigDecimal = 500f32.try_into().unwrap();
        let new_snapshot_numerable = NewSnapshotNumerable {
            snapshot_ptr_id: &inserted,
            quantity: &quantity,
            unit_value: &unit_value,
        };
        diesel::insert_into(crate::schema::finances_investments_snapshotnumerable::table)
            .values(&new_snapshot_numerable)
            .execute(conn)?;

        diesel::result::QueryResult::Ok(())
    })
    .unwrap();

    let all: Vec<SnapshotNumerable> = all_snapshotnumerable_for_account_id()
        .bind::<diesel::sql_types::Int8, _>(account_id)
        .load(&mut conn)
        .expect("Error loading snapshots");
    assert_eq!(all.len(), 3);
}
