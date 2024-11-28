//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in the same module.

pub mod account_list;

use crate::state::AppState;
use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{
    Account, AccountHolder, AccountHolderRole, AccountType, Custodian, Movement, MovementType, Snapshot, Transaction,
};
use finances_accounts::sql::filters::{
    account_by_pk, accountholder_by_pk, custodian_by_pk, movement_filter_account_by_pk, snapshot_filter_account_by_pk,
};
use tauri::State;

#[tauri::command]
pub async fn account_detail(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<crate::models::Account, String> {
    log::info!("Get Accounts pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all()
        .inner_join(
            finances_accounts::schema::finances_accounts_accountholderrole::table
                .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
        )
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((
            Account::as_select(),
            AccountHolderRole::as_select(),
            Custodian::as_select(),
            AccountType::as_select(),
        ))
        .filter(account_by_pk(pk))
        .first::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading account: {e}"))
        .map(|v| v.into())
}

#[tauri::command]
pub async fn account_snapshot_latest(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Option<crate::models::Snapshot>, String> {
    log::info!("Get (latest) Snapshot for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshot: Result<Option<Snapshot>, String> = Snapshot::all()
        .filter(snapshot_filter_account_by_pk(pk))
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
    pk: i64,
) -> Result<Vec<crate::models::Snapshot>, String> {
    log::info!("Get all Snapshots for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshots: Vec<Snapshot> = Snapshot::all()
        .filter(snapshot_filter_account_by_pk(pk))
        .load(&mut conn)
        .expect("Error returning all the snapshots");

    log::info!("Found {} snapshots for account pk {pk}", snapshots.len());
    Ok(snapshots.into_iter().map(|v| v.into()).collect())
}

#[tauri::command]
pub async fn account_movements(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Vec<crate::models::Movement>, String> {
    log::info!("Get all Movements for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let movements = Movement::all()
        .filter(movement_filter_account_by_pk(pk))
        .inner_join(Transaction::all())
        .inner_join(MovementType::all())
        .select((
            Movement::as_select(),
            Transaction::as_select(),
            MovementType::as_select(),
        ))
        .load::<(Movement, Transaction, MovementType)>(&mut conn)
        .expect("Error returning all the movements");

    log::info!("Found {} movements for account pk {pk}", movements.len());
    Ok(movements.into_iter().map(|v| v.into()).collect())
}

#[tauri::command]
pub async fn holders(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<crate::models::Holder>, String> {
    log::info!("Get all Holders in the database");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let holders = AccountHolder::all()
        .select(AccountHolder::as_select())
        .load::<AccountHolder>(&mut conn)
        .expect("Error returning all the holders");

    log::info!("Found {} holders", holders.len());
    Ok(holders.into_iter().map(|v| v.into()).collect())
}

#[tauri::command]
pub async fn holder_details(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<crate::models::Holder, String> {
    log::info!("Get Holder pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    AccountHolder::all()
        .filter(accountholder_by_pk(pk))
        .select(AccountHolder::as_select())
        .first::<AccountHolder>(&mut conn)
        .map_err(|e| format!("Error loading holder: {e}"))
        .map(|v| v.into())
}

#[tauri::command]
pub async fn custodian_details(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<crate::models::Custodian, String> {
    log::info!("Get Custodian pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Custodian::all()
        .filter(custodian_by_pk(pk))
        .select(Custodian::as_select())
        .first::<Custodian>(&mut conn)
        .map_err(|e| format!("Error loading custodian: {e}"))
        .map(|v| v.into())
}

#[tauri::command]
pub async fn base_media_url(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.base_media_url())
}

#[tauri::command]
pub async fn base_static_url(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.base_static_url())
}
