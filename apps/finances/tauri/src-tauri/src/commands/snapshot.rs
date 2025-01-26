use crate::types::ConnectionType;

use bigdecimal::BigDecimal;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::{Error, Result};
use diesel::prelude::*;
use finances_app_models::Snapshot as SnapshotProto;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
) -> Result<i64> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err(Error::Other("Error::RequestBodyMustBeRaw".to_string()));
    };

    let snapshot: SnapshotProto = data
        .to_owned()
        .try_into()
        .map_err(|e| Error::Other(format!("Failed to decode data to SnapshotProto: {e}")))?;

    log::info!("SnapshotProto: {:?}", snapshot);

    let mut conn = pool.get().expect("Get a connection from the Pool");

    insert_into_db(snapshot, &mut conn)
}

fn insert_into_db(snapshot: SnapshotProto, conn: &mut PgConnection) -> Result<i64> {
    let amount = snapshot.amount()?;

    let amount_value = amount.amount()?.amount();
    let date_value: chrono::NaiveDate = snapshot.date_value()?.try_into()?;
    let new_snapshot = finances_accounts::models::NewSnapshot {
        account_id: snapshot.account_pk(),
        amount: &amount_value,
        date_value: &date_value,
    };

    if let Some(numerable) = amount.as_numerable()? {
        let quantity: BigDecimal = numerable.quantity()?.try_into()?;
        let unit_value = numerable.unit_value()?.amount();
        let new_snapshot_numerable = finances_investments::models::NewSnapshotNumerable {
            new_snapshot: &new_snapshot,
            quantity: &quantity,
            unit_value: &unit_value,
        };
        Ok(new_snapshot_numerable.insert_into_db(conn)?)
    } else if let Some(_non_numerable) = amount.as_non_numerable()? {
        Ok(new_snapshot.insert_into_db(conn)?)
    } else {
        Err(Error::Other(
            "Nor numerable, neither non-numerable, can't do anything".to_string(),
        ))
    }
}
