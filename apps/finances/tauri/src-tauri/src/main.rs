// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let database_url = "postgres://finances_ro:finances_ro@localhost/finances";
    let pool = finances_app_lib::db::establish_connection(database_url);

    let builder = tauri::Builder::default();
    let app = finances_app_lib::create_app(builder, pool);

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}
