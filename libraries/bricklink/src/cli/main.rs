use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use tracing::debug;

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

    /// Path to a JSON file with user token
    #[clap(long)]
    token_file: Utf8PathBuf,
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

    Ok(())
}
