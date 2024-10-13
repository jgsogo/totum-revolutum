use crate::models::Snapshot;
use crate::test_utils::fixtures::database_with_accounts;
use crate::types::NumericType;
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
    let mut database_with_accounts = database_with_accounts();
    database_with_accounts.populate_snapshots(0).unwrap();

    // If there is a snapshot, I cannot remove the account
    {
        let r = diesel::delete(
            crate::schema::data_account::dsl::data_account.filter(crate::schema::data_account::id.eq(0)),
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

    // unique constraint on account-id and data: only one snapshot per account per day
    {
        let (acc_id, date_value) = crate::schema::data_snapshot::dsl::data_snapshot
            .select((
                crate::schema::data_snapshot::dsl::account_id,
                crate::schema::data_snapshot::dsl::date_value,
            ))
            .first::<(i32, chrono::NaiveDate)>(&mut database_with_accounts.conn)
            .expect("Error loading snapshots");

        let r = diesel::insert_into(crate::schema::data_snapshot::dsl::data_snapshot)
            .values((
                crate::schema::data_snapshot::amount.eq::<NumericType>(0.into()),
                crate::schema::data_snapshot::quantity.eq::<Option<i32>>(None),
                crate::schema::data_snapshot::unit_value.eq::<Option<NumericType>>(None),
                crate::schema::data_snapshot::date_value.eq(&date_value),
                crate::schema::data_snapshot::account_id.eq(acc_id),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::UniqueViolation));
                assert_eq!(
                    info.message(),
                    "UNIQUE constraint failed: data_snapshot.date_value, data_snapshot.account_id"
                );
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // Check quantity is positive
    {
        let r = diesel::insert_into(crate::schema::data_snapshot::dsl::data_snapshot)
            .values((
                crate::schema::data_snapshot::amount.eq::<NumericType>(0.into()),
                crate::schema::data_snapshot::quantity.eq::<Option<i32>>(Some(-3)),
                crate::schema::data_snapshot::unit_value.eq::<Option<NumericType>>(None),
                crate::schema::data_snapshot::date_value.eq(chrono::NaiveDate::from_ymd_opt(2250, 10, 13).unwrap()),
                crate::schema::data_snapshot::account_id.eq(0),
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
}

#[test]
fn test_required_fields() {
    let mut database_with_accounts = database_with_accounts();

    // Field account_id is required
    {
        let r = diesel::insert_into(crate::schema::data_snapshot::dsl::data_snapshot)
            .values((
                crate::schema::data_snapshot::amount.eq::<NumericType>(0.into()),
                crate::schema::data_snapshot::date_value.eq(chrono::NaiveDate::from_ymd_opt(2250, 10, 13).unwrap()),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_snapshot.account_id");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // Field amount is required
    {
        let r = diesel::insert_into(crate::schema::data_snapshot::dsl::data_snapshot)
            .values((
                crate::schema::data_snapshot::date_value.eq(chrono::NaiveDate::from_ymd_opt(2250, 10, 13).unwrap()),
                crate::schema::data_snapshot::account_id.eq(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_snapshot.amount");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }

    // Field date_value is required
    {
        let r = diesel::insert_into(crate::schema::data_snapshot::dsl::data_snapshot)
            .values((
                crate::schema::data_snapshot::amount.eq::<NumericType>(0.into()),
                crate::schema::data_snapshot::account_id.eq(0),
            ))
            .execute(&mut database_with_accounts.conn);

        assert!(r.is_err());
        let e = r.unwrap_err();
        match e {
            diesel::result::Error::DatabaseError(kind, info) => {
                assert!(matches!(kind, diesel::result::DatabaseErrorKind::NotNullViolation));
                assert_eq!(info.message(), "NOT NULL constraint failed: data_snapshot.date_value");
                assert!(info.table_name().is_none());
                assert!(info.constraint_name().is_none());
            }
            _ => panic!("Test failed!"),
        }
    }
}
