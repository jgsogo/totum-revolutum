// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL envvar is required");
    let pool = finances_app_lib::db::establish_connection(&database_url);

    let builder = tauri::Builder::default();
    let app = finances_app_lib::create_app(builder, pool);

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}
