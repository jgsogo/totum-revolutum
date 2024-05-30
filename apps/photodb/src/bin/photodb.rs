use anyhow::anyhow;
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use std::str::FromStr;
use tracing::{debug, error};

use pcloud_sdk::cli::auth;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::types::RemotePath;
use photodb::db::PCloudDatabase;
use photodb::PhotoDB;

fn application_dir() -> Utf8PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to get dirs::home_dir()");
    let photodb_dir = home_dir.join(".photodb");
    Utf8PathBuf::from_path_buf(photodb_dir).unwrap()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Commands,

    /// Path to application directory. Temporary files and secrets will be stored here
    #[clap(long, default_value_t = application_dir())]
    app_dir: Utf8PathBuf,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Authenticate using inputs from command line
    Auth(auth::AuthParams),

    /// Authenticate using inputs from file
    AuthFile(auth::AuthFileParams),

    /// Initializes the database (fails if file already exists)
    Initialize,

    /// Adds (and backups) a photo to the database
    Add(Add),
}

#[derive(Args, Debug)]
struct Add {
    photo_file: Utf8PathBuf,
}

fn tracing_level(log_level: log::LevelFilter) -> tracing::Level {
    match log_level {
        log::LevelFilter::Off => tracing::Level::ERROR,
        log::LevelFilter::Error => tracing::Level::ERROR,
        log::LevelFilter::Warn => tracing::Level::WARN,
        log::LevelFilter::Info => tracing::Level::INFO,
        log::LevelFilter::Debug => tracing::Level::DEBUG,
        log::LevelFilter::Trace => tracing::Level::TRACE,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Configure tracing - logs go to stderr so it can be separated from actual output
    let tracing_level = tracing_level(cli.verbose.log_level_filter());
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_level)
        .init();
    debug!("Tracing level configured to {}", tracing_level);

    std::fs::create_dir_all(&cli.app_dir)?;
    let token_file = cli.app_dir.join(".pcloud");
    let db_path = RemotePath::from_str("path:/developing")?;
    let app_dir = cli.app_dir;
    let app_dir_db = app_dir.join("db");
    std::fs::create_dir_all(&app_dir_db)?; // Create it here, we don't want to do anything unless we are sure that we will be able to back up the DB in case of error.

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Auth(input) => auth::handle_auth(&token_file, input).await?,
        Commands::AuthFile(input) => auth::handle_auth_file(&token_file, input).await?,
        _ => {
            let token = auth::read_from_file::<OAuth2TokenImpl, &Utf8PathBuf>(&token_file)?;
            let client = PCloudClientImpl::new(token, true);
            let done = match &cli.command {
                Commands::Initialize => PCloudDatabase::initialize(client, db_path).await?,
                _ => {
                    let (db, done) = PCloudDatabase::new(client.clone(), db_path.clone()).await?;
                    let photodb = PhotoDB::new(db, client, app_dir, db_path);
                    match cli.command {
                        Commands::Add(add) => {
                            photodb.add(add.photo_file)?;
                        }
                        c => error!("Unexpected command {:?}", c),
                    };
                    done
                }
            };
            debug!("Await for proxied-file upload to finish");
            if let Err((_tmpdir, localfile)) = done.await? {
                error!("Failed to execute cleanup task (upload) of proxied file. We save the DB to a local file");
                let date = chrono::Local::now();
                let db_filename = format!("{}.sqlite3", date.format("%Y-%m-%d][%H:%M:%S"));
                let db_backup_filename = app_dir_db.join(db_filename);
                std::fs::copy(&localfile, &db_backup_filename).map_err(|e| {
                    anyhow!(
                        "Failed to back up from temporal file '{:?}'. Database changes are lost: {e}",
                        localfile
                    )
                })?;
                error!("Local DB has been backed up into filename '{db_backup_filename}'. Execute the application again to run auto-recovery");
            }
            debug!("Proxied-file upload finished")
        }
    };
    Ok(())
}
