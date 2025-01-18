use crate::types::ConnectionType;
use bigdecimal::ToPrimitive;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_app_models::{AppState, NewTransaction};
use tauri::State;

#[tauri::command]
pub fn create_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
    state: State<'_, AppState>,
) -> Result<f32, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let new_transaction: NewTransaction = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to NewTransaction: {e}"))?;

    let mut conn = pool.get().expect("Get a connection from the Pool");
    new_transaction
        .insert_into_db(&mut conn, state.base_ccy())
        .map_err(|e| format!("Error saving transaction to db: {e}"))?
        .to_f32()
        .ok_or("Error converting BigDecimal to f32".to_string())
}
