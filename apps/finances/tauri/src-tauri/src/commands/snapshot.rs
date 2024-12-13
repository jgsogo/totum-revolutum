use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::Account;
use finances_accounts::models::NewSnapshot;
use finances_accounts::sql::filters::account_by_pk;
use finances_investments::models::NewSnapshotNumerable;
use log::info;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    snapshot: crate::models::NewSnapshot,
) -> Result<i64, String> {
    info!("Create snapshot: {snapshot:?}");

    let mut conn = pool.get().expect("Get a connection from the Pool");
    let account_numerable = Account::all()
        .select(finances_accounts::schema::finances_accounts_account::is_numerable)
        .filter(account_by_pk(snapshot.account_pk))
        .first::<bool>(&mut conn)
        .map_err(|e| format!("Error loading account: {e}"))?;

    let date_value = chrono::NaiveDate::parse_from_str(&snapshot.date_value, "%Y-%m-%d")
        .map_err(|e| format!("Error parsing date from string ({}): {e}", snapshot.date_value))?;

    let (amount, quantity, unit_value) = snapshot.amount.into_bigdecimals(account_numerable)?;

    let new_snapshot = NewSnapshot {
        account_id: &snapshot.account_pk,
        amount: &amount,
        date_value: &date_value,
    };

    let r = if account_numerable {
        let new_snapshot_numerable = NewSnapshotNumerable {
            new_snapshot: &new_snapshot,
            quantity: quantity.as_ref().expect("Already validated"),
            unit_value: unit_value.as_ref().expect("Already validated"),
        };
        new_snapshot_numerable.insert_into_db(&mut conn)
    } else {
        new_snapshot.insert_into_db(&mut conn)
    };

    r.map_err(|e| format!("Error inserting snapshot to database: {e}"))
}
