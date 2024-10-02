//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in this same module.

pub mod menu;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType, Snapshot};
use tauri::State;

#[tauri::command]
pub async fn account_detail_command(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    pk: i32,
) -> Result<crate::models::Account, String> {
    log::info!("Get Accounts pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let r = Account::get_with_holder_and_type(pk)
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .first::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts");

    Ok(r.into())
}

#[tauri::command]
pub async fn account_snapshot_command(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    pk: i32,
) -> Result<Option<crate::models::Snapshot>, String> {
    log::info!("Get (latest) Snapshot for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshot: Option<Snapshot> = Snapshot::all_snapshots(pk)
        .first(&mut conn)
        .optional()
        .expect("Error returning the last snapshot");
    match snapshot {
        Some(snapshot) => Ok(Some(snapshot.into())),
        None => Ok(None),
    }
}
