pub mod add;
pub mod list;
use std::path::Path;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// List all crons
    List,

    /// Add a new cron for a directory
    Add(add::AddParams),
}

pub async fn handle(home: &Path, input: &Commands) {
    match &input {
        Commands::List => {
            list::handle(home);
        }
        Commands::Add(input) => {
            add::handle(home, input);
        }
    }
}
