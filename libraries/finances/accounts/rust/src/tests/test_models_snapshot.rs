use crate::models::{NewSnapshot, Snapshot};
use crate::test_utils::establish_connection;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let pool = establish_connection();
    let mut conn = pool.get().unwrap();

    let account_id = 0i64;
    // All snapshots
    {
        let all_snapshots = Snapshot::all()
            .filter(crate::schema::finances_accounts_snapshot::account_id.eq(account_id))
            .select(Snapshot::as_select())
            .load::<Snapshot>(&mut conn)
            .expect("Error counting 'all' snapshots");
        assert_eq!(all_snapshots.len(), 2);
    }

    // Add a snapshot
    {
        let amount: bigdecimal::BigDecimal = 3f32.try_into().unwrap();
        let date_value = chrono::NaiveDate::parse_from_str("2022-11-30", "%Y-%m-%d").unwrap();
        let new_snapshot = NewSnapshot {
            amount: &amount,
            date_value: &date_value,
            account_id: &account_id,
        };
        let inserted = diesel::insert_into(crate::schema::finances_accounts_snapshot::table)
            .values(&new_snapshot)
            .execute(&mut conn)
            .unwrap();
        assert_eq!(inserted, 1);

        // ... and now we have one more snapshot
        let all_snapshots = Snapshot::all()
            .filter(crate::schema::finances_accounts_snapshot::account_id.eq(account_id))
            .select(Snapshot::as_select())
            .load::<Snapshot>(&mut conn)
            .expect("Error counting 'all' snapshots");
        assert_eq!(all_snapshots.len(), 3);
    }
}
