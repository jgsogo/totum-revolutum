use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use tracing::debug;

use pcloud_sdk::client::HttpClient;

use crate::output::{OutputArg, PrintVariant};

mod auth;
mod download;
mod listfolder;
mod output;
mod userinfo;
mod utils;
mod upload;

/// Arguments that apply to all subcommands
#[derive(Parser)]
pub struct CliParams {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// If the command allows it, number of parallel task to run
    #[clap(long, default_value_t = 3)]
    parallel: usize,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    common: CliParams,

    /// Path to a JSON file with user token
    #[clap(long)]
    token_file: Utf8PathBuf,

    #[clap(value_enum, long, default_value_t=OutputArg::Default)]
    output: OutputArg,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Authenticate using inputs from command line
    Auth(auth::AuthParams),

    /// Authenticate using inputs from file
    AuthFile(auth::AuthFileParams),

    Userinfo,

    Listfolder(listfolder::Params),

    Download(download::Params),

    Upload(upload::Params),
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

    // Configure tracing - logs go to stderr so it can be separated from actual output
    let tracing_level = tracing_level(cli.common.verbose.log_level_filter());
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_level)
        .init();
    debug!("Tracing level configured to {}", tracing_level);

    // Get the output
    let output: PrintVariant = PrintVariant::new(cli.output, &cli.common);

    // Go ahead!
    match &cli.command {
        Commands::Auth(input) => auth::handle_auth(&cli.token_file, &output, input).await,
        Commands::AuthFile(input) => auth::handle_auth_file(&cli.token_file, input).await,
        _ => {
            let token = auth::read_from_file(&cli.token_file)?;
            let client = HttpClient::new(token, true);
            match cli.command {
                Commands::Userinfo => userinfo::handle(client, &output, cli.common).await,
                Commands::Listfolder(params) => listfolder::handle(client, &output, params, cli.common).await,
                Commands::Download(params) => download::handle(client, &output, params, cli.common).await,
                Commands::Upload(params) => upload::handle(client, &output, params, cli.common).await,
                c => bail!("Unexpected command {:?}", c),
            }
        }
    }
}
