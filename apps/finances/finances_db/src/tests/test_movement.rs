use crate::models::{Movement, MovementType, Transfer};
use crate::test_utils::fixtures::database_with_accounts;
use crate::types::NumericType;
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
    let mut database_with_accounts = database_with_accounts();
    database_with_accounts.populate_transfers().unwrap();
    database_with_accounts.populate_movements(0).unwrap();

    let (movement_id, account_id, fx_id, type_id, transfer_id) = crate::schema::data_movement::dsl::data_movement
        .select((
            crate::schema::data_movement::dsl::id,
            crate::schema::data_movement::dsl::account_id,
            crate::schema::data_movement::dsl::fx_id,
            crate::schema::data_movement::dsl::type_id,
            crate::schema::data_movement::dsl::transfer_id,
        ))
        .first::<(i32, i32, Option<i32>, i32, i32)>(&mut database_with_accounts.conn)
        .expect("Error loading movement");

    // If there is a movement, I cannot remove the account
    {
        let r = diesel::delete(
            crate::schema::data_account::dsl::data_account.filter(crate::schema::data_account::id.eq(account_id)),
        )
        .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::Unknown));
                assert_eq!(info.message(), "FOREIGN KEY constraint failed");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // If there is a movement, I cannot remove the FX
    {
        let r =
            diesel::delete(crate::schema::data_fx::dsl::data_fx.filter(crate::schema::data_fx::id.eq(fx_id.unwrap())))
                .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::Unknown));
                assert_eq!(info.message(), "FOREIGN KEY constraint failed");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // If there is a movement, I cannot remove the movmeent type
    {
        let r = diesel::delete(
            crate::schema::data_movementtype::dsl::data_movementtype
                .filter(crate::schema::data_movementtype::id.eq(type_id)),
        )
        .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::Unknown));
                assert_eq!(info.message(), "FOREIGN KEY constraint failed");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // Check quantity is positive
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                quantity.eq::<Option<i32>>(Some(-2)),
                unit_value.eq::<Option<NumericType>>(None),
                direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                fx_id.eq::<Option<i32>>(None),
                transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::CheckViolation));
                assert_eq!(info.message(), "CHECK constraint failed: quantity_positive");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // if transfer is removed, all associated movements are removed as well
    {
        let r = diesel::delete(
            crate::schema::data_transfer::dsl::data_transfer.filter(crate::schema::data_transfer::id.eq(transfer_id)),
        )
        .execute(&mut database_with_accounts.conn);

        assert!(r.is_ok());

        let movement_exists = diesel::select(diesel::dsl::exists(
            crate::schema::data_movement::dsl::data_movement
                .filter(crate::schema::data_movement::dsl::id.eq(movement_id)),
        ))
        .get_result(&mut database_with_accounts.conn);
        assert_eq!(movement_exists, Ok(false));
    }
}

#[test]
fn test_required_fields() {
    let mut database_with_accounts = database_with_accounts();
    database_with_accounts.populate_transfers().unwrap();

    // 'amount' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                // amount.eq::<NumericType>(0.into()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.amount");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // 'date_value' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                // date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.date_value");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // 'account_id' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                // account_id.eq(0),
                direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.account_id");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // 'direction' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                // direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.direction");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // 'date' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                direction.eq(0),
                // date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.date");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // 'transfer_id' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                // transfer_id.eq::<i32>(0),
                type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.transfer_id");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // 'type_id' is required
    {
        use crate::schema::data_movement::dsl::*;
        let r = diesel::insert_into(crate::schema::data_movement::dsl::data_movement)
            .values((
                amount.eq::<NumericType>(0.into()),
                date_value.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                account_id.eq(0),
                direction.eq(0),
                date.eq(chrono::NaiveDate::from_ymd_opt(2024, 9, 6).unwrap()),
                transfer_id.eq::<i32>(0),
                // type_id.eq::<i32>(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_movement.type_id");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }
}
