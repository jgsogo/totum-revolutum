use crate::types::ConnectionType;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, Snapshot};
use tauri::State;

#[tauri::command]
pub async fn get_account_details(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<crate::models::Account, String> {
    log::info!("Get Accounts pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::details_for_pk(pk, &mut conn)
        .map_err(|e| format!("Error loading account: {e}"))
        .map(|v| v.into())
}

#[tauri::command]
pub async fn get_account_snapshot_latest(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Option<crate::models::Snapshot>, String> {
    log::info!("Get (latest) Snapshot for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    match Account::latest_snapshot_for_pk(pk, &mut conn) {
        Ok(snapshot) => Ok(snapshot.map(|v| v.into())),
        Err(e) => Err(format!("Error loading last snapshot: {e}")),
    }
}

#[tauri::command]
pub async fn get_account_snapshots(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Vec<crate::models::Snapshot>, String> {
    log::info!("Get all Snapshots for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshots: Vec<Snapshot> = Account::snapshots_for_pk(pk, &mut conn).expect("Error returning all the snapshots");

    log::info!("Found {} snapshots for account pk {pk}", snapshots.len());
    Ok(snapshots.into_iter().map(|v| v.into()).collect())
}

#[tauri::command]
pub async fn get_account_movements(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Vec<crate::models::Movement>, String> {
    log::info!("Get all Movements for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let movements = Account::movements_for_pk(pk, &mut conn).expect("Error returning all the movements");

    log::info!("Found {} movements for account pk {pk}", movements.len());
    Ok(movements.into_iter().map(|v| v.into()).collect())
}
