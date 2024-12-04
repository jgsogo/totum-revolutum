use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::AccountHolder;
use finances_accounts::sql::filters::accountholder_by_pk;
use tauri::State;

#[tauri::command]
pub async fn holders(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<crate::models::Holder>, String> {
    log::info!("Get all Holders in the database");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let holders = AccountHolder::all()
        .select(AccountHolder::as_select())
        .order(finances_accounts::schema::finances_accounts_accountholder::name.asc())
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
