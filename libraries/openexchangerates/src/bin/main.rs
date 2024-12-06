use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use openexchangerates::OXRClient;
use tracing::{debug, info};

/// Arguments that apply to all subcommands
#[derive(Parser)]
pub struct CliParams {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    common: CliParams,

    /// App ID (Go to https://docs.openexchangerates.org/reference/api-introduction to get one)
    #[clap(long)]
    app_id: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get the latest exchange rates available from the Open Exchange Rates API
    Latest(BaseAndSymbols),

    /// Get basic plan information and usage statistics for an Open Exchange Rates App ID
    Usage,

    /// Get historical exchange rates for any date available from the Open Exchange Rates API, currently going back to 1st January 1999.
    Historical(HistoricalArgs),

    /// Get a JSON list of all currency symbols available from the Open Exchange Rates API, along with their full names, for use in your integration.
    Currencies,
}

#[derive(Args, Debug)]
pub struct HistoricalArgs {
    /// The requested date in YYYY-MM-DD format (required).
    #[arg(value_parser = parse_date)]
    date: chrono::NaiveDate,

    #[clap(flatten)]
    base_and_symbols: BaseAndSymbols,
}

fn parse_date(arg: &str) -> Result<chrono::NaiveDate, chrono::format::ParseError> {
    chrono::NaiveDate::parse_from_str(arg, "%Y-%m-%d")
}

#[derive(Args, Debug)]
pub struct BaseAndSymbols {
    /// Change base currency (3-letter code, default: USD)
    #[arg(long)]
    base: Option<String>,

    /// Limit results to specific currencies (comma-separated list of 3-letter codes)
    #[arg(long)]
    symbols: Option<String>,
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

    // Configure tracing - logs go to stderr, so it can be separated from actual output
    let tracing_level = tracing_level(cli.common.verbose.log_level_filter());
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_level)
        .init();
    debug!("Tracing level configured to {}", tracing_level);

    let client = OXRClient::new(cli.app_id)?;

    // Go ahead!
    match &cli.command {
        Commands::Latest(input) => {
            let latest = client.latest(input.base.as_ref(), input.symbols.as_ref()).await?;
            println!("{:?}", latest);
            Ok(())
        }
        Commands::Usage => {
            let data = client.usage().await?;
            println!("{:?}", data);
            Ok(())
        }
        Commands::Historical(input) => {
            let data = client
                .historical(
                    &input.date,
                    input.base_and_symbols.base.as_ref(),
                    input.base_and_symbols.symbols.as_ref(),
                )
                .await?;
            println!("{:?}", data);
            Ok(())
        }
        Commands::Currencies => {
            let data = client.currencies().await?;
            println!("{:?}", data);
            Ok(())
        }
    }
}
