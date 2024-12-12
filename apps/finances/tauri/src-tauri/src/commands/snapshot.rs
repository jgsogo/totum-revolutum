use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::Account;
use finances_accounts::models::NewSnapshot;
use finances_accounts::sql::filters::account_by_pk;
use finances_investments::managers::create_snapshot_numerable;
use log::info;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    snapshot: crate::models::NewSnapshot,
) -> Result<usize, String> {
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

    let r = if account_numerable {
        // Insert one more snapshot numerable
        create_snapshot_numerable(
            &mut conn,
            &snapshot.account_pk,
            &amount,
            &date_value,
            quantity.as_ref().expect("Already validated"),
            unit_value.as_ref().expect("Already validated"),
        )
    } else {
        let new_snapshot = NewSnapshot {
            account_id: &snapshot.account_pk,
            amount: &amount,
            date_value: &date_value,
        };

        diesel::insert_into(finances_accounts::schema::finances_accounts_snapshot::table)
            .values(&new_snapshot)
            .execute(&mut conn)
    };

    r.map_err(|e| format!("Error inserting snapshot to database: {e}"))
}
