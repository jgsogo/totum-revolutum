use crate::types::ConnectionType;

use bigdecimal::BigDecimal;
use diesel::r2d2::{ConnectionManager, Pool};

use finances_app_models::Snapshot as SnapshotProto;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
) -> Result<i64, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let snapshot: SnapshotProto = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to SnapshotProto: {e}"))?;

    log::info!("SnapshotProto: {:?}", snapshot);

    let mut conn = pool.get().expect("Get a connection from the Pool");

    // Insert into the database
    let amount = snapshot.amount().map_err(|e| e.to_string())?;

    let amount_value = amount.amount().map_err(|e| e.to_string())?.amount();
    let date_value: chrono::NaiveDate = snapshot
        .date_value()
        .map_err(|e| e.to_string())?
        .try_into()
        .map_err(|e: finances_app_models::errors::ConversionError| e.to_string())?;
    let new_snapshot = finances_accounts::models::NewSnapshot {
        account_id: snapshot.pk(),
        amount: &amount_value,
        date_value: &date_value,
    };

    if let Some(numerable) = amount.as_numerable().map_err(|e| e.to_string())? {
        let quantity: BigDecimal = numerable
            .quantity()
            .map_err(|e| e.to_string())?
            .try_into()
            .map_err(|e: finances_app_models::errors::ConversionError| e.to_string())?;
        let unit_value = numerable.unit_value().map_err(|e| e.to_string())?.amount();
        let new_snapshot_numerable = finances_investments::models::NewSnapshotNumerable {
            new_snapshot: &new_snapshot,
            quantity: &quantity,
            unit_value: &unit_value,
        };
        new_snapshot_numerable
            .insert_into_db(&mut conn)
            .map_err(|e| format!("Error saving snapshot numerable to db: {e}"))
    } else if let Some(_non_numerable) = amount.as_non_numerable().map_err(|e| e.to_string())? {
        new_snapshot
            .insert_into_db(&mut conn)
            .map_err(|e| format!("Error saving snapshot to db: {e}"))
    } else {
        Err("Nor numerable, neither non-numerable, can't do anything".to_string())
    }
}
