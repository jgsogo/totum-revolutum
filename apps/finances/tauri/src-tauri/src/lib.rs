use diesel::r2d2::{ConnectionManager, Pool};
use tauri::Manager;
pub mod commands;
pub mod db;
pub mod models;
pub mod state;
mod types;
mod views;

use crate::types::ConnectionType;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn create_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    db_pool: Pool<ConnectionManager<ConnectionType>>,
    state: state::AppState,
) -> tauri::App<R> {
    // TODO: See mutability example in the App::manage method. It shows how to update the connection. Of course we don't want here a hardcoded pool. User may want to switch to different DBs

    builder
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .build(),
        )
        .setup(|app| {
            app.manage(db_pool);
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::account_list::all_accounts,
            commands::account_list::savings_accounts,
            commands::account_list::investment_accounts,
            commands::account_list::retirement_accounts,
            commands::snapshot::create_snapshot,
            commands::account::account_detail,
            commands::account::account_snapshot_latest,
            commands::account::account_snapshots,
            commands::account::account_movements,
            commands::holder::holders,
            commands::holder::holder_details,
            commands::custodian::custodian_details,
            commands::base_media_url,
            commands::base_static_url,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}
