use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::TransactionGroup;
// use finances_accounts::sql::filters::custodian_by_pk;
use tauri::State;

#[tauri::command]
pub async fn get_all_transaction_groups(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<crate::models::TransactionGroup>, String> {
    log::info!("Get all transaction groups");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let groups = TransactionGroup::all()
        .select(TransactionGroup::as_select())
        .load::<TransactionGroup>(&mut conn)
        .map_err(|e| format!("Error loading transactions groups: {e}"))?;

    Ok(groups
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::TransactionGroup>>())
}
