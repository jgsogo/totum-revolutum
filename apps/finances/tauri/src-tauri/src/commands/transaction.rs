use crate::types::ConnectionType;
use crate::{Error, Result};
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_app_models::{
    google_type, AppState as AppStateProto, Movement as MovementProto, Transaction as TransactionProto,
};
use tauri::State;

#[tauri::command]
pub fn create_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
    state: State<'_, AppStateProto>,
) -> std::result::Result<f32, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let transaction: TransactionProto = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to NewTransaction: {e}"))?;

    let mut conn = pool.get().expect("Get a connection from the Pool");
    insert_into_db(transaction, &mut conn, state.base_ccy().map_err(|e| e.to_string())?)
        .map(|v| v.to_f32().unwrap())
        .map_err(|e| e.to_string())
}

fn insert_into_db(
    transaction: TransactionProto,
    conn: &mut PgConnection,
    base_ccy: google_type::CurrencyCode,
) -> Result<BigDecimal> {
    let (total_from, total_to) = conn.transaction(|conn| {
        // Create the transaction
        let transaction_pk = {
            let new_transaction = finances_accounts::models::NewTransaction {
                name: transaction.name(),
                description: transaction.description(),
                group_id: transaction.group().map(|g| g.pk()),
            };
            new_transaction.insert_into_db(conn)?
        };

        let total_from: BigDecimal = transaction
            .movements_from()
            .map(|mov| insert_movement_into_db(transaction_pk, mov, conn, base_ccy))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(BigDecimal::zero(), |sum, mov_amount| sum + mov_amount);

        let total_to: BigDecimal = transaction
            .movements_to()
            .map(|mov| insert_movement_into_db(transaction_pk, mov, conn, base_ccy))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(BigDecimal::zero(), |sum, mov_amount| sum + mov_amount);

        Ok::<_, Error>((total_from, total_to))
    })?;

    if !finances_accounts::types::compare_eq(&total_from, &total_to) {
        Err(Error::Other(format!(
            "Mismatched amounts, from {total_from} != to {total_to}"
        )))
    } else {
        Ok(total_from)
    }
}

fn insert_movement_into_db(
    transaction_pk: i64,
    movement: &MovementProto,
    conn: &mut PgConnection,
    base_ccy: google_type::CurrencyCode,
) -> Result<BigDecimal> {
    todo!()
}
