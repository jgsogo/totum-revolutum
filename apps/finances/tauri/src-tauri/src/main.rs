// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use finances_app_models::{google_type, AppState, DatabaseConnection};
use openexchangerates::OXRClient;

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

        let base_url = std::env::var("DJANGO_BASE_URL").expect("DJANGO_BASE_URL envvar is required");
        let media_url = std::env::var("MEDIA_URL").expect("MEDIA_URL envvar is required");
        let static_url = std::env::var("STATIC_URL").expect("STATIC_URL envvar is required");
        let backup_folder = std::env::var("BACKUP_DIRECTORY")
            .map(|s| std::path::PathBuf::from(&s))
            .unwrap_or_else(|_| std::env::temp_dir());
        let backup_folder = camino::Utf8PathBuf::from_path_buf(backup_folder).expect("Invalid tmp folder");
        let base_ccy = google_type::CurrencyCode::EUR;
        AppState::new(
            base_ccy,
            media_url,
            static_url,
            base_url,
            postgres_db,
            &backup_folder,
            None,
        )
    };
    let pool =
        finances_app_lib::db::establish_connection(&state.db().expect("Database connection missing").postgres_url());

    let builder = tauri::Builder::default();
    let initial_holder_pk = 48; // TODO: We don't want to hardcode the initial holder here
    let oxr_client = {
        let api_key = std::env::var("OPENEXCHANGERATES_APIKEY").expect("OPENEXCHANGERATES_APIKEY envvar is required");
        OXRClient::new(api_key).unwrap()
    };
    let app = finances_app_lib::create_app(builder, pool, state, initial_holder_pk, Some(oxr_client));

    log::debug!("Run application");
    app.run(|_app_handle, _event| {
        // if let tauri::RunEvent::ExitRequested { api, .. } = event {
        //     api.prevent_exit();
        // }
    });
}
