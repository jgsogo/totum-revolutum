use crate::types::ConnectionType;

use diesel::r2d2::{ConnectionManager, Pool};
use finances_app_models::NewSnapshot;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
) -> Result<i64, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let new_snapshot: NewSnapshot = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to NewSnapshot: {e}"))?;

    let mut conn = pool.get().expect("Get a connection from the Pool");
    let new_snapshot_pk = new_snapshot
        .insert_into_db(&mut conn)
        .map_err(|e| format!("Error saving snapshot to db: {e}"))?;

    Ok(new_snapshot_pk)
}
