// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use camino::Utf8PathBuf;
use clap::{Args, Parser};
use clap_stdin::FileOrStdin;
use finances_app_models::{google_type, AppState, DatabaseConnection};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to configuration file
    #[clap(long, default_value = "-")]
    config: FileOrStdin,
    // TODO: If not provided, go to dirs::home_dir / .finances / config.toml
    // TODO: Implement this logic as something reusable
}

#[derive(Deserialize, Debug)]
struct Config {
    db: DbConfig,
    tauri: TauriConfig,
    django: DjangoConfig,
}

#[derive(Deserialize, Debug)]
struct DbConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct TauriConfig {
    backup_directory: Utf8PathBuf,
}

#[derive(Deserialize, Debug)]
struct DjangoConfig {
    base_url: String,
    media_url: String,
    static_url: String,
}

fn main() {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);
    // println!("cli={}", cli.config.contents().expect("lolo"));
    let config: Config = toml::from_str(&cli.config.contents().expect("lolo")).expect("lolailo");
    println!("config: {:?}", config);

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
        let backup_folder = std::env::var("BACKUP_DIRECTORY")
            .map(|s| std::path::PathBuf::from(&s))
            .unwrap_or_else(|_| std::env::temp_dir());
        let backup_folder = camino::Utf8PathBuf::from_path_buf(backup_folder).expect("Invalid tmp folder");
        AppState::new(
            google_type::CurrencyCode::EUR,
            media_url,
            static_url,
            base_url,
            postgres_db,
            &backup_folder,
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
