use crate::types::ConnectionType;
use diesel::r2d2::{ConnectionManager, Pool};
use log::info;
use tauri::State;

#[tauri::command]
pub fn create_snapshot(
    _pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    account_pk: i64,
    date_value: String,
    amount: Option<f32>,
    quantity: Option<i32>,
    unit_value: Option<f32>,
) -> Result<(), String> {
    info!("Create snapshot for account {account_pk}: date_value: {date_value}, amount: {amount:?}, quantity: {quantity:?}, unit_value: {unit_value:?}");
    Err("Backend side not implemented".to_string())
}
