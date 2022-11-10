use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};
mod auth;
use home;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authorize pCloud application
    Auth(auth::AuthParams),
}

fn pcloud_home() -> PathBuf {
    match env::var("PCLOUD_HOME_DIR") {
        Ok(p) => {
            let p = PathBuf::from(&p);
            if p.is_relative() {
                eprintln!(
                    "PCLOUD_HOME_DIR ('{}') needs to be an absolute path",
                    p.display()
                );
                std::process::exit(1);
            }
            p
        }
        Err(_) => match home::home_dir() {
            Some(mut h) => {
                h.push(".pcloud");
                h
            }
            None => panic!("Provide home directory for pCloud"),
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    let pcloud_home = pcloud_home();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Auth(input)) => {
            auth::handle_auth(input);
        }
        None => {
            println!("Default subcommand");
        }
    }

    Ok(())
}
