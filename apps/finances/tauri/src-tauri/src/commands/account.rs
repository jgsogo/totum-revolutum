use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::Account;
use finances_accounts::sql::filters::account_by_pk;
use finances_investments::models::SnapshotNumerable;
use finances_investments::sql::queries::all_snapshotnumerable_for_account_id;
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

    let account_numerable = Account::all()
        .select(finances_accounts::schema::finances_accounts_account::is_numerable)
        .filter(account_by_pk(pk))
        .first::<bool>(&mut conn)
        .map_err(|e| format!("Error loading account: {e}"))?;

    if !account_numerable {
        match Account::latest_snapshot_for_pk(pk, &mut conn) {
            Ok(snapshot) => Ok(snapshot.map(|v| v.into())),
            Err(e) => Err(format!("Error loading last snapshot: {e}")),
        }
    } else {
        let snapshot_numerable: Option<SnapshotNumerable> = all_snapshotnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(pk)
            .load(&mut conn)
            .map_err(|e| format!("Error loading snapshots numerable: {e}"))?
            .into_iter()
            .next();

        match snapshot_numerable {
            Some(snapshot) => Ok(Some(snapshot.into())),
            None => Ok(None),
        }
    }
}

#[tauri::command]
pub async fn get_account_snapshots(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Vec<crate::models::Snapshot>, String> {
    log::info!("Get all Snapshots for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let account_numerable = Account::all()
        .select(finances_accounts::schema::finances_accounts_account::is_numerable)
        .filter(account_by_pk(pk))
        .first::<bool>(&mut conn)
        .map_err(|e| format!("Error loading account: {e}"))?;

    if !account_numerable {
        Ok(Account::snapshots_for_pk(pk, &mut conn)
            .map_err(|e| format!("Error retrieving snapshots: {e}"))?
            .into_iter()
            .map(|v| v.into())
            .collect())
    } else {
        Ok(all_snapshotnumerable_for_account_id()
            .bind::<diesel::sql_types::Int8, _>(pk)
            .load(&mut conn)
            .map_err(|e| format!("Error loading snapshots numerable: {e}"))?
            .into_iter()
            .map(|v: SnapshotNumerable| v.into())
            .collect())
    }
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
