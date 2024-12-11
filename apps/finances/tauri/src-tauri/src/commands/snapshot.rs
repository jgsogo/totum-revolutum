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

    let amount: bigdecimal::BigDecimal = snapshot.amount.ok_or("Amount expected")?.try_into().map_err(|e| {
        format!(
            "Cannot convert amount f32 ({}) to BigDecimal: {e}",
            snapshot.amount.unwrap()
        )
    })?;
    let date_value = chrono::NaiveDate::parse_from_str(&snapshot.date_value, "%Y-%m-%d")
        .map_err(|e| format!("Error parsing date from string ({}): {e}", snapshot.date_value))?;

    let r = if account_numerable {
        // Insert one more snapshot numerable
        let quantity: bigdecimal::BigDecimal =
            snapshot.quantity.ok_or("Quantity expected")?.try_into().map_err(|e| {
                format!(
                    "Cannot convert quantity f32 ({}) to BigDecimal: {e}",
                    snapshot.quantity.unwrap()
                )
            })?;

        let unit_value: bigdecimal::BigDecimal =
            snapshot
                .unit_value
                .ok_or("UnitValue expected")?
                .try_into()
                .map_err(|e| {
                    format!(
                        "Cannot convert unit_value f32 ({}) to BigDecimal: {e}",
                        snapshot.unit_value.unwrap()
                    )
                })?;

        create_snapshot_numerable(
            &mut conn,
            &snapshot.account_pk,
            &amount,
            &date_value,
            &quantity,
            &unit_value,
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
