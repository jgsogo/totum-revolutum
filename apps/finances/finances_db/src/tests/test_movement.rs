use crate::models::{Movement, MovementType, Transfer};
use crate::test_utils::fixtures::database_with_accounts;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut database_with_accounts = database_with_accounts();
    database_with_accounts.populate_transfers().unwrap();
    database_with_accounts.populate_movements(0).unwrap();
    database_with_accounts.populate_movements(2).unwrap();

    // Account without movements
    {
        let all: i64 = Movement::all_with_related_data(1)
            .count()
            .get_result(&mut database_with_accounts.conn)
            .expect("Error loading movements");

        assert_eq!(all, 0);
    }

    // Account with movements
    {
        let all = Movement::all_with_related_data(0)
            .select((Movement::as_select(), Transfer::as_select(), MovementType::as_select()))
            .load::<(Movement, Transfer, MovementType)>(&mut database_with_accounts.conn)
            .expect("Error loading movements");

        assert_eq!(all.len(), 2);

        let (latest, _, _) = all.get(0).unwrap();
        let (next, _, _) = all.get(1).unwrap();
        assert!(latest.date_value > next.date_value); // Movements are ordered, first one is the latest one
        assert!(latest.date > next.date); // Movements are ordered, first one is the latest one
    }
}

#[test]
fn test_constraints() {
    // TODO: If there is a movement, I cannot remove the account

    // TODO: If there is a movement, I cannot remove the FX

    // TODO: If there is a movement, I cannot remove the movmeent type

    // TODO: Check quantity is positive

    // Movement type
    // TODO: Check movementtype level is positive

    // TODO: Check movementtype parent is set to null if parent is removed

    // Transfer
    // TODO: if transfer is removed, all associated movements are removed as well
}
