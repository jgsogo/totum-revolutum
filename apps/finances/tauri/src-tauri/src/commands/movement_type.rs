use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::managers;
use finances_accounts::models::MovementType;
use finances_accounts::sql::filters::movementtype_by_pk;
use tauri::State;

#[tauri::command]
pub fn get_all_movementtypes(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<crate::models::MovementType>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let movementtypes = MovementType::all()
        .select(MovementType::as_select())
        .load::<MovementType>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    Ok(movementtypes
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::MovementType>>())
}

#[tauri::command]
pub fn get_breadcrumbs_for_movementtype(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    pk: i64,
) -> Result<Vec<String>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let movementtype = MovementType::all()
        .filter(movementtype_by_pk(pk))
        .select(MovementType::as_select())
        .first::<MovementType>(&mut conn)
        .map_err(|e| format!("Error loading movement type: {}", e))?;

    managers::get_breadcrumbs_for_movementtype(&mut conn, &movementtype)
        .map_err(|e| format!("Error getting the breadcrumbs: {}", e))
}
