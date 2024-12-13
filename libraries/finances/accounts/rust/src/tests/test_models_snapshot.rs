use super::DB_POOL;
use crate::models::{NewSnapshot, Snapshot};
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

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
        new_snapshot.insert_into_db(&mut conn).unwrap();

        // ... and now we have one more snapshot
        let all_snapshots = Snapshot::all()
            .filter(crate::schema::finances_accounts_snapshot::account_id.eq(account_id))
            .select(Snapshot::as_select())
            .load::<Snapshot>(&mut conn)
            .expect("Error counting 'all' snapshots");
        assert_eq!(all_snapshots.len(), 3);
    }
}
