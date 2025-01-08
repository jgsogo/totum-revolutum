//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in the same module.

pub mod account;
pub mod account_list;
pub mod custodian;
pub mod holder;
pub mod movement_type;
pub mod snapshot;
pub mod transaction;
pub mod transaction_group;
use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::AccountHolder;
use finances_app_models::{AppModel, AppState, Holder, MainContext};
use prost::Message;
use tauri::ipc::Response;
use tauri::State;

#[tauri::command]
pub async fn get_app_state(state: State<'_, AppState>) -> Result<Response, String> {
    let encoded = state.encode_to_vec();
    Ok(Response::new(encoded))
}

#[tauri::command]
pub async fn get_main_context(pool: State<'_, Pool<ConnectionManager<ConnectionType>>>) -> Result<Response, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let holders = AccountHolder::all()
        .select(AccountHolder::as_select())
        .order(finances_accounts::schema::finances_accounts_accountholder::name.asc())
        .load::<AccountHolder>(&mut conn)
        .map_err(|e| format!("Error loading holders: {}", e))?;

    log::debug!("Found {} holders", holders.len());

    let main_context = MainContext::new(holders);
    let encoded = main_context.encode_to_vec();
    Ok(Response::new(encoded))
}
