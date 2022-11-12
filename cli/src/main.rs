use clap::{Parser, Subcommand};
mod app;
pub mod data;
mod home;
use tracing::debug;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage pCloud applications
    #[command(subcommand)]
    App(app::Commands),

    /// Print home folder
    Home,
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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        Some(Commands::App(input)) => {
            app::handle(&pcloud_home, input);
        }
        Some(Commands::Home) => {
            home::handle(&pcloud_home);
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}
