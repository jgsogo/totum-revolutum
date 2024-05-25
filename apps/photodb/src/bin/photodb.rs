use anyhow::bail;
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use std::str::FromStr;
use std::time;
use tracing::debug;

use pcloud_sdk::cli::auth;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::types::RemotePath;
use photodb::db::PCloudDatabase;

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

    /// Adds files to myapp
    Add(Add),
}

#[derive(Args, Debug)]
struct Add {
    name: Option<String>,
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

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let r = match &cli.command {
        Commands::Auth(input) => auth::handle_auth(&token_file, input).await,
        Commands::AuthFile(input) => auth::handle_auth_file(&token_file, input).await,
        _ => {
            let token = auth::read_from_file::<OAuth2TokenImpl, &Utf8PathBuf>(&token_file)?;
            let client = PCloudClientImpl::new(token, true);
            match &cli.command {
                Commands::Initialize => PCloudDatabase::initialize(client, &cli.app_dir, db_path).await,
                _ => {
                    let _db = PCloudDatabase::new(client, &cli.app_dir, db_path).await?;
                    match cli.command {
                        Commands::Add(add) => {
                            println!("Commands::Add({add:?})");
                            Ok(())
                        }
                        c => bail!("Unexpected command {:?}", c),
                    }
                }
            }
        }
    };
    // FIXME: We need to sleep here so the DB is uploaded again to pcloud. We need to implement a better alternative: send a signal back once the work is done
    debug!("Sleep for a while");
    std::thread::sleep(time::Duration::from_secs(5));
    r
}
