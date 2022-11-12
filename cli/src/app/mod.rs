pub mod auth;
pub mod list;
use std::path::Path;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Authorize pCloud application
    Auth(auth::AuthParams),

    /// List all applications
    List(list::ListParams),
}

pub async fn handle(home: &Path, input: &Commands) {
    match &input {
        Commands::Auth(input) => {
            auth::handle(home, input).await;
        }
        Commands::List(input) => {
            list::handle(home, input);
        }
    }
}
