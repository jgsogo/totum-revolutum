use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::Account;
use finances_accounts::models::NewSnapshot;
use finances_accounts::sql::filters::account_by_pk;
use log::info;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    account_pk: i64,
    date_value: String,
    amount: Option<f32>,
    quantity: Option<i32>,
    unit_value: Option<f32>,
) -> Result<(), String> {
    info!("Create snapshot for account {account_pk}: date_value: {date_value}, amount: {amount:?}, quantity: {quantity:?}, unit_value: {unit_value:?}");

    let mut conn = pool.get().expect("Get a connection from the Pool");

    let account_numerable = Account::all()
        .select(finances_accounts::schema::finances_accounts_account::is_numerable)
        .filter(account_by_pk(account_pk))
        .first::<bool>(&mut conn)
        .map_err(|e| format!("Error loading account: {e}"))?;

    if account_numerable {
        Err("TODO: Backend side not implemented".to_string())
    } else {
        let amount: bigdecimal::BigDecimal = amount
            .ok_or("Amount expected")?
            .try_into()
            .map_err(|e| format!("Cannot convert f32 ({}) to BigDecimal: {e}", amount.unwrap()))?;
        let new_snapshot = NewSnapshot {
            account_id: &account_pk,
            amount: &amount,
            date_value: &chrono::NaiveDate::parse_from_str(&date_value, "%Y-%m-%d")
                .map_err(|e| format!("Error parsing date from string ({date_value}): {e}"))?,
        };

        diesel::insert_into(finances_accounts::schema::finances_accounts_snapshot::table)
            .values(&new_snapshot)
            .execute(&mut conn)
            .map_err(|e| format!("Error inserting snapshot to database: {e}"))?;
        Ok(())
    }
}
