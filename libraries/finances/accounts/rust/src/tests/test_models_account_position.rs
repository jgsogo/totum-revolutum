use super::DB_POOL;
use crate::fields::MovementDirection;
use crate::models::{Account, MovementType, NewMovement, NewSnapshot, Transaction};
use crate::sql::filters::movementtype_by_unique_name;
use crate::types::NumericType;
use bigdecimal::Zero;
use diesel::prelude::*;

#[test]
fn test_queries() {
    let mut conn = DB_POOL.get().unwrap();

    let account_id = 1i64;
    let transaction_id = Transaction::all()
        .select(crate::schema::finances_accounts_transaction::id)
        .first::<i64>(&mut conn)
        .unwrap();
    let movement_type_id = MovementType::all()
        .filter(movementtype_by_unique_name(crate::constants::movementtype::EXPENSE))
        .select(crate::schema::finances_accounts_movementtype::id)
        .get_result::<i64>(&mut conn)
        .unwrap();

    let account = Account::from_pk(account_id, &mut conn).unwrap();
    // Starting with an account with no snapshots and no movements
    {
        let all_snapshots = Account::snapshots_for_pk(account_id, &mut conn).unwrap();
        assert_eq!(all_snapshots.len(), 0);

        let all_movements = Account::movements_for_pk(account_id, &mut conn).unwrap();
        assert_eq!(all_movements.len(), 0);
    }

    // Without movements and snapshots, the position is zero
    {
        let date_value = chrono::NaiveDate::parse_from_str("2024-12-01", "%Y-%m-%d").unwrap();
        let position = account.position(&mut conn, &date_value).unwrap();
        assert_eq!(position, NumericType::zero());
    }

    // If we add one snapshot, then the position will match it
    let snapshot_date_value = chrono::NaiveDate::parse_from_str("2024-12-01", "%Y-%m-%d").unwrap();
    let snapshot_amount: bigdecimal::BigDecimal = 3f32.try_into().unwrap();
    {
        let new_snapshot = NewSnapshot {
            amount: &snapshot_amount,
            date_value: &snapshot_date_value,
            account_id: &account_id,
        };
        new_snapshot.insert_into_db(&mut conn).unwrap();

        // For a date matching the snapshots, it is the same amount
        let position = account.position(&mut conn, &snapshot_date_value).unwrap();
        assert_eq!(position, snapshot_amount);

        // The previous date, it is zero
        let position = account
            .position(&mut conn, &(snapshot_date_value - chrono::Days::new(1)))
            .unwrap();
        assert_eq!(position, NumericType::zero());

        // The day after, it matches the snapshot
        let position = account
            .position(&mut conn, &(snapshot_date_value + chrono::Days::new(1)))
            .unwrap();
        assert_eq!(position, snapshot_amount);
    }

    // Now we add a movement before the snapshot
    {
        let movement_date_value = chrono::NaiveDate::parse_from_str("2024-11-01", "%Y-%m-%d").unwrap();
        let movement_amount: bigdecimal::BigDecimal = 1f32.try_into().unwrap();
        let new_movement = NewMovement {
            amount: &movement_amount,
            direction: MovementDirection::In,
            date_value: &movement_date_value,
            account_id: &account_id,
            fx_id: None,
            type_id: &movement_type_id,
            transaction_id: &transaction_id,
        };
        new_movement.insert_into_db(&mut conn).unwrap();

        // For a date matching the snapshots, it is the same amount
        let position = account.position(&mut conn, &snapshot_date_value).unwrap();
        assert_eq!(position, snapshot_amount);

        // The previous date, it takes the movement into account
        let position = account
            .position(&mut conn, &(snapshot_date_value - chrono::Days::new(1)))
            .unwrap();
        assert_eq!(position, movement_amount);

        // The day after, it matches the snapshot
        let position = account
            .position(&mut conn, &(snapshot_date_value + chrono::Days::new(1)))
            .unwrap();
        assert_eq!(position, snapshot_amount);

        // The day for the movement, it takes the movement into account
        let position = account.position(&mut conn, &movement_date_value).unwrap();
        assert_eq!(position, movement_amount);

        // The day before the movement, it is zero
        let position = account
            .position(&mut conn, &(movement_date_value - chrono::Days::new(1)))
            .unwrap();
        assert_eq!(position, NumericType::zero());
    }

    // Now we add a movement OUT before the snapshot
    {
        let movement_date_value = chrono::NaiveDate::parse_from_str("2024-11-05", "%Y-%m-%d").unwrap();
        let movement_amount: bigdecimal::BigDecimal = 2f32.try_into().unwrap();
        let new_movement = NewMovement {
            amount: &movement_amount,
            direction: MovementDirection::Out,
            date_value: &movement_date_value,
            account_id: &account_id,
            fx_id: None,
            type_id: &movement_type_id,
            transaction_id: &transaction_id,
        };
        new_movement.insert_into_db(&mut conn).unwrap();

        // The date before the snapshot, it takes the movement into account
        let position = account
            .position(&mut conn, &(snapshot_date_value - chrono::Days::new(1)))
            .unwrap();
        assert_eq!(
            position,
            <f32 as TryInto<bigdecimal::BigDecimal>>::try_into(-1f32).unwrap()
        );
    }

    // If we add another snapshot at the very beginning
    {
        let date_value = chrono::NaiveDate::parse_from_str("2024-10-01", "%Y-%m-%d").unwrap();
        let amount: bigdecimal::BigDecimal = 100f32.try_into().unwrap();
        let new_snapshot = NewSnapshot {
            amount: &amount,
            date_value: &date_value,
            account_id: &account_id,
        };
        new_snapshot.insert_into_db(&mut conn).unwrap();

        // The date before the original snapshot, it takes the movements (and this new snapshot) into account
        let position = account
            .position(&mut conn, &(snapshot_date_value - chrono::Days::new(1)))
            .unwrap();
        assert_eq!(
            position,
            <f32 as TryInto<bigdecimal::BigDecimal>>::try_into(99f32).unwrap()
        );
    }
}
