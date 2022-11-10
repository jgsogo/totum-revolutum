use clap::{Parser, Subcommand};
mod app;
mod home;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    let pcloud_home = home::pcloud_home();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
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
