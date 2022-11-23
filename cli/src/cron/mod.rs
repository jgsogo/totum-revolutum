pub mod add;
pub mod list;
use anyhow::Result;
use clap::Subcommand;
use std::path::Path;

#[derive(Subcommand)]
pub enum Commands {
    /// List all crons
    List,

    /// Add a new cron for a directory
    Add(add::AddParams),
}

pub fn handle(home: &Path, input: &Commands) -> Result<()> {
    match &input {
        Commands::List => {
            list::handle(home);
        }
        Commands::Add(input) => {
            add::handle(home, input)?;
        }
    };
    Ok(())
}
