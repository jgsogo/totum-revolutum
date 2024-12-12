//! Provides some helper queries that operate on actual instance (diesel is hidden):
//! these functions takes data and return model instances.

use crate::models::movement_numerable::_NewMovementNumerable;
use crate::models::{NewMovementNumerable, NewSnapshotNumerable};
use diesel::prelude::*;
use finances_accounts::models::NewSnapshot;
use finances_accounts::types::NumericType;

pub fn create_snapshot_numerable(
    conn: &mut PgConnection,
    account_id: &i64,
    amount: &NumericType,
    date_value: &chrono::NaiveDate,
    quantity: &NumericType,
    unit_value: &NumericType,
) -> Result<usize, diesel::result::Error> {
    // Insert one more snapshot numerable
    conn.transaction(|conn| {
        let new_snapshot = NewSnapshot {
            amount,
            date_value,
            account_id,
        };
        let inserted = diesel::insert_into(finances_accounts::schema::finances_accounts_snapshot::table)
            .values(&new_snapshot)
            .returning(finances_accounts::schema::finances_accounts_snapshot::id)
            .get_result(conn)?;

        let new_snapshot_numerable = NewSnapshotNumerable {
            snapshot_ptr_id: &inserted,
            quantity,
            unit_value,
        };
        diesel::insert_into(crate::schema::finances_investments_snapshotnumerable::table)
            .values(&new_snapshot_numerable)
            .execute(conn)
    })
}

pub fn create_movement_numerable(
    conn: &mut PgConnection,
    new_movement_numerable: &NewMovementNumerable,
) -> Result<usize, diesel::result::Error> {
    // Insert one more movement numerable
    conn.transaction(|conn| {
        let inserted = diesel::insert_into(finances_accounts::schema::finances_accounts_movement::table)
            .values(new_movement_numerable.new_movement)
            .returning(finances_accounts::schema::finances_accounts_movement::id)
            .get_result(conn)?;

        let new_movement_numerable = _NewMovementNumerable {
            movement_ptr_id: &inserted,
            quantity: new_movement_numerable.quantity,
            unit_value: new_movement_numerable.unit_value,
        };
        diesel::insert_into(crate::schema::finances_investments_movementnumerable::table)
            .values(&new_movement_numerable)
            .execute(conn)
    })
}
