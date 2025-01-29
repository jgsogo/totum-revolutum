// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use finances_app_models::{google_type, AppState, DatabaseConnection};

fn main() {
    let state = {
        let postgres_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL envvar is required");
        let base_url = std::env::var("BASE_URL").expect("BASE_URL envvar is required");
        let media_url = std::env::var("MEDIA_URL").expect("MEDIA_URL envvar is required");
        let static_url = std::env::var("STATIC_URL").expect("STATIC_URL envvar is required");
        let db = DatabaseConnection::new(postgres_url);
        AppState::new(google_type::CurrencyCode::EUR, media_url, static_url, base_url, db)
    };
    let pool = finances_app_lib::db::establish_connection(state.postgres_url().expect("postgres_url missing"));

    let builder = tauri::Builder::default();
    let app = finances_app_lib::create_app(builder, pool, state);

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}
