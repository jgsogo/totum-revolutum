use anyhow::{anyhow, bail, Result};
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use filesystem::impls::FilesystemPCloud;
use filesystem::{FilesystemRead, FilesystemWrite};
use std::str::FromStr;
use tracing::{debug, error};

use pcloud_sdk::cli::auth;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::types::RemotePath;
use photodb::db::{Database, PCloudDatabase};
use photodb::{AppDirs, PhotoDB};

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

    /// Syncs the database with the remote storage
    Sync(Sync),
}

#[derive(Args, Debug)]
struct Add {
    photo_file: Utf8PathBuf,
}

#[derive(Args, Debug)]
struct Sync {
    /// Remove DB entries that are no longer in the remote storage
    #[clap(long, default_value_t = true)]
    remove_missing_files: bool,

    /// Add entries to the DB for new files discovered in the remote
    #[clap(long, default_value_t = true)]
    collect_new_files: bool,
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

/// Any command that uses the DB is executed here. This way we can guarantee that the Receiver work
/// (store the database back to pCloud if anything fails) is always executed
async fn db_commands<T: Database, RemoteStorage: FilesystemRead + FilesystemWrite>(
    command: Commands,
    photodb: PhotoDB<'_, T, RemoteStorage>,
) -> Result<()> {
    match command {
        Commands::Add(add) => photodb.add(add.photo_file).await,
        Commands::Sync(sync) => {
            photodb.sync(sync.collect_new_files, sync.remove_missing_files).await?;
            Ok(())
        }
        c => bail!("Unexpected command {:?}", c),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
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
    let app_dir = AppDirs::new(cli.app_dir)?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Auth(input) => auth::handle_auth(&token_file, input).await?,
        Commands::AuthFile(input) => auth::handle_auth_file(&token_file, input).await?,
        _ => {
            let token = auth::read_from_file::<OAuth2TokenImpl, &Utf8PathBuf>(&token_file)?;
            let client = PCloudClientImpl::new(token, true);

            // I can't raise from these commands, as I always need to execute the backup routine
            let done = match &cli.command {
                Commands::Initialize => PCloudDatabase::initialize(client, db_path).await?,
                _ => {
                    let remote_storage = FilesystemPCloud::new(db_path.clone(), client.clone()).await?;
                    let (db, done) = PCloudDatabase::new(client, db_path).await?;
                    let r = match PhotoDB::new(db, remote_storage, &app_dir).await {
                        Ok(photodb) => db_commands(cli.command, photodb).await,
                        Err(e) => Err(e),
                    };

                    if let Err(e) = r {
                        error!("{}", e);
                    }

                    done
                }
            };
            debug!("Await for proxied-file upload to finish");
            if let Err((_tmpdir, localfile)) = done.await? {
                error!("Failed to execute cleanup task (upload) of proxied file. We save the DB to a local file");
                let date = chrono::Local::now();
                let db_filename = format!("{}.sqlite3", date.format("%Y-%m-%d][%H:%M:%S"));
                let db_backup_filename = app_dir.db_backups().join(db_filename);
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
