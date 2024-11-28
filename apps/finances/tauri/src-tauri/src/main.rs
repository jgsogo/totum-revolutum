// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use finances_app_lib::state::AppState;

fn main() {
    let state = AppState::default();
    let pool = finances_app_lib::db::establish_connection(state.postgres_url());

    let builder = tauri::Builder::default();
    let app = finances_app_lib::create_app(builder, pool, state);

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}
