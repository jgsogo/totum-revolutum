use anyhow::bail;
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use std::str::FromStr;
use tracing::debug;

use pcloud_sdk::cli::auth;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::types::RemotePath;
use photodb::db::PCloudDatabase;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Commands,

    /// Path to a JSON file with user token
    #[clap(long, default_value_t = Utf8PathBuf::from_path_buf(dirs::home_dir().expect("Cannot get dirs::config_dir()").join(".photodb/.pcloud")).expect("Failed to get Utf8Path from dirs::config_dir()"))]
    token_file: Utf8PathBuf,
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

    let db_path = RemotePath::from_str("path:/developing")?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Auth(input) => auth::handle_auth(&cli.token_file, input).await,
        Commands::AuthFile(input) => auth::handle_auth_file(&cli.token_file, input).await,
        _ => {
            let token = auth::read_from_file::<OAuth2TokenImpl, &Utf8PathBuf>(&cli.token_file)?;
            let client = PCloudClientImpl::new(token, true);
            match &cli.command {
                Commands::Initialize => PCloudDatabase::initialize(client, db_path).await,
                _ => {
                    let _db = PCloudDatabase::new(client, db_path).await?;
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
    }
}
