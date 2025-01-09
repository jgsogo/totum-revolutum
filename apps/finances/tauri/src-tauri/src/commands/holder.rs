use crate::types::ConnectionType;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::AccountHolder;

use tauri::State;

#[tauri::command]
pub async fn get_holder_details(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<crate::models::Holder, String> {
    log::info!("Get Holder pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    AccountHolder::from_pk(pk, &mut conn)
        .map_err(|e| format!("Error loading holder: {e}"))
        .map(|v| v.into())
}
