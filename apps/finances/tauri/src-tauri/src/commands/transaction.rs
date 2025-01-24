use crate::types::ConnectionType;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_app_models::{google_type, AppState as AppStateProto, Transaction as TransactionProto};
use tauri::State;

#[tauri::command]
pub fn create_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
    state: State<'_, AppStateProto>,
) -> Result<f32, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let transaction: TransactionProto = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to NewTransaction: {e}"))?;

    let mut conn = pool.get().expect("Get a connection from the Pool");
    insert_into_db(transaction, &mut conn, state.base_ccy().map_err(|e| e.to_string())?).map(|v| v.to_f32().unwrap())
}

fn insert_into_db(
    transaction: TransactionProto,
    conn: &mut PgConnection,
    base_ccy: google_type::CurrencyCode,
) -> Result<BigDecimal, String> {
    todo!("not impl -- Do we need the NewTransaction proto? And what about the NewMovement?")
}
