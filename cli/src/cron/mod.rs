pub mod list;
use std::path::Path;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// List all crons
    List,
}

pub async fn handle(home: &Path, input: &Commands) {
    match &input {
        Commands::List => {
            list::handle(home);
        }
    }
}
