use crate::models::Snapshot;
use crate::test_utils::fixtures::database_with_accounts;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut database_with_accounts = database_with_accounts();
    database_with_accounts.populate_snapshots(0).unwrap();
    database_with_accounts.populate_snapshots(2).unwrap();

    // Account without snapshots
    {
        let all: i64 = Snapshot::all_snapshots(1)
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading snapshots");

        assert_eq!(all, 0);
    }

    // Account with snapshots
    {
        let all = Snapshot::all_snapshots(0)
            .select(Snapshot::as_select())
            .load::<Snapshot>(&mut database_with_accounts.conn)
            .expect("Error loading snapshots");

        assert_eq!(all.len(), 2);

        let latest = all.get(0).unwrap();
        let next = all.get(1).unwrap();
        assert!(latest.date_value > next.date_value); // Snapshots are ordered, first one is the latest one
    }
}

#[test]
fn test_constraints() {
    // TODO: If there is a snapshot, I cannot remove the account

    // TODO: unique constraint on account-id and data: only one snapshot per account per day

    // TODO: Check quantity is positive
}
