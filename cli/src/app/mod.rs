pub mod auth;
pub mod list;
use anyhow::Result;
use clap::Subcommand;
use std::path::Path;

#[derive(Subcommand)]
pub enum Commands {
    /// Authorize pCloud application
    Auth(auth::AuthParams),

    /// List all applications
    List(list::ListParams),
}

pub async fn handle(home: &Path, input: &Commands) -> Result<()> {
    match &input {
        Commands::Auth(input) => {
            auth::handle(home, input).await?;
        }
        Commands::List(input) => {
            list::handle(home, input);
        }
    };
    Ok(())
}
