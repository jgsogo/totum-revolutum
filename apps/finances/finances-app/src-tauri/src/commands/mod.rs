//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in the same module.

pub mod menu;

use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType, Movement, MovementType, Snapshot, Transfer};
use tauri::State;

#[tauri::command]
pub async fn account_detail(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i32,
) -> Result<crate::models::Account, String> {
    log::info!("Get Accounts pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::get_with_holder_and_type(pk)
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .first::<(Account, AccountHolder, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading account: {e}"))
        .map(|v| v.into())
}

#[tauri::command]
pub async fn account_snapshot_latest(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i32,
) -> Result<Option<crate::models::Snapshot>, String> {
    log::info!("Get (latest) Snapshot for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshot: Result<Option<Snapshot>, String> = Snapshot::all_snapshots(pk)
        .first(&mut conn)
        .optional()
        .map_err(|e| format!("Error loading last snapshot: {e}"));

    match snapshot {
        Ok(snapshot) => Ok(snapshot.map(|v| v.into())),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn account_snapshots(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i32,
) -> Result<Vec<crate::models::Snapshot>, String> {
    log::info!("Get all Snapshots for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshots: Vec<Snapshot> = Snapshot::all_snapshots(pk)
        .load(&mut conn)
        .expect("Error returning all the snapshots");

    log::info!("Found {} snapshots for account pk {pk}", snapshots.len());
    Ok(snapshots.into_iter().map(|v| v.into()).collect())
}

#[tauri::command]
pub async fn account_movements(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i32,
) -> Result<Vec<crate::models::Movement>, String> {
    log::info!("Get all Movements for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let movements = Movement::all_with_related_data(pk)
        .select((
            Movement::as_select(),
            // Fx::as_select(),
            Transfer::as_select(),
            MovementType::as_select(),
        ))
        .load::<(Movement, Transfer, MovementType)>(&mut conn)
        .expect("Error returning all the movements");

    log::info!("Found {} movements for account pk {pk}", movements.len());
    Ok(movements.into_iter().map(|v| v.into()).collect())
}
