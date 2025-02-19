// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use finances_app_models::{google_type, AppState, DatabaseConnection};

fn main() {
    let state = {
        let postgres_db = {
            let user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER envvar is required");
            let password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD envvar is required");
            let host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST envvar is required");
            let port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT envvar is required");
            let port = port.parse::<u16>().expect("POSTGRES_PORT not u16");
            let dbname = std::env::var("POSTGRES_DB").expect("POSTGRES_DB envvar is required");
            DatabaseConnection::new(&user, &password, &host, port, &dbname)
        };

        let base_url = std::env::var("BASE_URL").expect("BASE_URL envvar is required");
        let media_url = std::env::var("MEDIA_URL").expect("MEDIA_URL envvar is required");
        let static_url = std::env::var("STATIC_URL").expect("STATIC_URL envvar is required");
        AppState::new(
            google_type::CurrencyCode::EUR,
            media_url,
            static_url,
            base_url,
            postgres_db,
        )
    };
    let pool =
        finances_app_lib::db::establish_connection(&state.db().expect("Database connection missing").postgres_url());

    let builder = tauri::Builder::default();
    let app = finances_app_lib::create_app(builder, pool, state);

    app.run(|_app_handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
}
