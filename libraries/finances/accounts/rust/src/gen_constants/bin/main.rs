
use anyhow::{anyhow, bail, Result};
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use tracing::{debug, error, info};
use finances_accounts::models::{AccountType, MovementType};
use diesel::prelude::*;
use std::fs::File;
use std::io::Write;

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Database URL
    #[clap(long)]
    database_url: String,

    /// Output file to generate
    #[clap(long)]
    output_file: Utf8PathBuf,
}

pub fn establish_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
    match PgConnection::establish(&database_url) {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("Could not connect to PostgreSQL.");
            error!("Error connecting to {}", database_url);
            Err(e)
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Configure tracing - logs go to stderr so it can be separated from actual output
    let tracing_level = tracing_level(cli.verbose.log_level_filter());
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_level)
        .init();
    debug!("Tracing level configured to {}", tracing_level);

    let mut conn = establish_connection(&cli.database_url)?;
    let mut output = File::create(cli.output_file).expect("Unable to create file");
    write!(output, "// This file is auto-generated. Do not modify\n\n");
    write!(output, "// We use 'unique_name' fields to identify these elements because the PK might be different depending on the status of the DB when the data is migrated\n\n");

    // AccountType
    {
        let account_types =  AccountType::all_with_unique_name()
            .select(AccountType::as_select())
            .load(&mut conn)
            .expect("Error loading account types");

        write!(output, "pub mod accounttype {{\n");
        for account_type in account_types {
            let unique_name = account_type.unique_name.unwrap();
            let unique_name_var = unique_name.trim_start_matches('/').replace("/", "_").replace("-", "_").to_uppercase();
            write!(output, "    pub const {}: &str = \"{}\";\n", unique_name_var, unique_name);
        }
        write!(output, "}}\n")?;
    }

    // MovementType
    {
        let mov_types =  MovementType::all_with_unique_name()
            .select(MovementType::as_select())
            .load(&mut conn)
            .expect("Error loading account types");

        write!(output, "pub mod movementtype {{\n");
        for mov_type in mov_types {
            let unique_name = mov_type.unique_name.unwrap();
            let unique_name_var = unique_name.trim_start_matches('/').replace("/", "_").replace("-", "_").to_uppercase();
            write!(output, "    pub const {}: &str = \"{}\";\n", unique_name_var, unique_name);
        }
        write!(output, "}}\n")?;
    }

    Ok(())
}
