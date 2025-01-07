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

use crate::state::AppState;
use finances_app_models::protos;
use prost::Message;
use tauri::ipc::Response;
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

#[tauri::command]
pub async fn get_app_config(state: State<'_, AppState>) -> Result<Response, String> {
    let app_config = protos::AppConfig {
        base_ccy: protos::Ccy::from_str_name(&state.base_ccy)
            .ok_or(format!("Provided CCY '{}' not in enum", state.base_ccy))?
            .into(),
        base_media_url: state.base_media_url(),
        base_static_url: state.base_static_url(),
        base_url: state.base_url().to_string(),
        db: None,
    };

    let encoded = app_config.encode_to_vec();
    Ok(Response::new(encoded))
}
