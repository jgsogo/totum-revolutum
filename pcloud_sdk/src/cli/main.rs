use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::debug;

mod auth;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Path to a JSON file with user token
    #[clap(long)]
    token_file: std::path::PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate using inputs from command line
    Auth(auth::AuthParams),

    /// Authenticate using inputs from file
    AuthFile(auth::AuthFileParams),
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
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Configure tracing
    let tracing_level = tracing_level(cli.verbose.log_level_filter());
    tracing_subscriber::fmt().with_max_level(tracing_level).init();
    debug!("Tracing level configured to {}", tracing_level);

    // Get the token
    // let _token = get_user_token(cli.secrets_file, cli.token_file, false).await?;

    // Go ahead!
    match &cli.command {
        Commands::Auth(input) => auth::handle_auth(&cli.token_file, input).await,
        Commands::AuthFile(input) => auth::handle_auth_file(&cli.token_file, input).await,
    }
}
