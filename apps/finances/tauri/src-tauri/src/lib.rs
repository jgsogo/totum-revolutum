use diesel::r2d2::{ConnectionManager, Pool};
use tauri::Manager;
pub mod commands;
pub mod db;
mod menu;
pub mod models;
mod types;
mod views;

use crate::types::ConnectionType;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn create_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    db_pool: Pool<ConnectionManager<ConnectionType>>,
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
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::menu::sidebar_menu,
            commands::account_detail,
            commands::account_snapshot_latest,
            commands::account_snapshots,
            commands::account_movements,
            commands::holders,
            commands::holder_details,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}
