use diesel::r2d2::{ConnectionManager, Pool};
use tauri::Manager;
pub mod commands;
pub mod db;
pub mod models;
mod types;
mod views;
use finances_app_models::AppState;

use crate::types::ConnectionType;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn create_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    db_pool: Pool<ConnectionManager<ConnectionType>>,
    state: AppState,
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
            commands::account_list::get_all_accounts,
            commands::account_list::get_all_accounts_for_holder,
            commands::account_list::get_all_savings_accounts_for_holder,
            commands::account_list::get_all_investment_accounts_for_holder,
            commands::account_list::get_all_retirement_accounts_for_holder,
            commands::account::get_account_details,
            commands::account::get_account_snapshot_latest,
            commands::account::get_account_snapshots,
            commands::account::get_account_movements,
            commands::holder::get_holder_details,
            commands::custodian::get_custodian_details,
            commands::get_app_state,
            commands::get_main_context,
            commands::movement_type::get_all_movementtypes,
            commands::movement_type::get_breadcrumbs_for_movementtype,
            commands::transaction_group::get_all_transaction_groups,
            // Sending data
            commands::snapshot::create_snapshot,
            commands::transaction::create_transaction,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}
