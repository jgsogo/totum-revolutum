use std::path::Path;

use anyhow::Result;
use clap::Subcommand;

pub mod add;
pub mod list;

#[derive(Subcommand)]
pub enum Commands {
    /// List all crons
    List,

    /// Add a new cron for a directory
    Add(add::AddParams),
}

pub fn handle(home: &Path, input: &Commands) -> Result<()> {
    match &input {
        Commands::List => list::handle(home),
        Commands::Add(input) => add::handle(home, input),
    }
}
