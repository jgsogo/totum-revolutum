use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::debug;

mod app;
mod cron;
mod home;
mod init;
mod status;
mod common;
mod output;
mod run;
mod errors;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage pCloud applications
    #[command(subcommand)]
    App(app::Commands),

    /// Cron management for registered folders
    #[command(subcommand)]
    Cron(cron::Commands),

    /// Initialize pCloud folder
    Init(init::InitParams),

    /// Show status for pCloud folder
    Status(status::StatusParams),

    /// Print home folder
    Home,

    /// Run syncronization
    Run(run::RunParams),
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
    tracing_subscriber::fmt()
        .with_max_level(tracing_level)
        .init();
    debug!("Tracing level configured to {}", tracing_level);

    // Go ahead!
    let pcloud_home = home::pcloud_home();
    match &cli.command {
        Commands::App(input) => app::handle(&pcloud_home, input).await,
        Commands::Cron(input) => cron::handle(&pcloud_home, input),
        Commands::Home => home::handle(&pcloud_home),
        Commands::Init(input) => init::handle(&pcloud_home, input),
        Commands::Status(input) => status::handle(&pcloud_home, input),
        Commands::Run(input) => run::handle(&pcloud_home, input).await,
    }
}
