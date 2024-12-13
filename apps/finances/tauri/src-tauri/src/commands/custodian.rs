use crate::types::ConnectionType;

use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::Custodian;

use tauri::State;

#[tauri::command]
pub async fn get_custodian_details(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<crate::models::Custodian, String> {
    log::info!("Get Custodian pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Custodian::from_pk(pk, &mut conn)
        .map_err(|e| format!("Error loading custodian: {e}"))
        .map(|v| v.into())
}
