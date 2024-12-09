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

use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_base_url(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.base_url().to_string())
}

#[tauri::command]
pub async fn get_base_media_url(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.base_media_url())
}

#[tauri::command]
pub async fn get_base_static_url(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.base_static_url())
}

#[tauri::command]
pub async fn get_base_ccy(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.base_ccy.clone())
}
