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

pub fn handle(_home: &Path, input: &Commands) {
    match &input {
        Commands::Auth(input) => {
            auth::handle(input);
        }
        Commands::List(input) => {
            list::handle(input);
        }
    }
}
