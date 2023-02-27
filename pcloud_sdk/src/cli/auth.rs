use std::fs;
use std::fs::File;
use std::io::BufReader;

use anyhow::{anyhow, bail, Result};
use clap::Args;
use serde::de::DeserializeOwned;
use tracing::info;

use pcloud_sdk::methods::oauth2;
use pcloud_sdk::methods::oauth2::{AppClientData, OAuth2TokenImpl};

use crate::output::PrintVariant;

#[derive(Args, Debug)]
pub struct AuthParams {
    #[clap(long)]
    /// Client id
    client_id: String,

    #[clap(long, group = "password")]
    /// Client secret
    client_secret: Option<String>,

    #[clap(long, action, group = "password")]
    /// Take the client secret from stdin
    client_secret_stdin: bool,
}

#[derive(Args, Debug)]
pub struct AuthFileParams {
    /// Path to a JSON file with client secrets
    #[clap(long)]
    secrets_file: std::path::PathBuf,
}

pub fn read_from_file<T: DeserializeOwned, P: AsRef<std::path::Path>>(path: P) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}

async fn get_user_token(app_client_data: AppClientData) -> Result<OAuth2TokenImpl> {
    let address = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let client = reqwest::Client::new();
    oauth2::authorize_oauth2::<OAuth2TokenImpl>(client, app_client_data, address).await
}

fn write_to_file(token_file: &std::path::Path, token: &OAuth2TokenImpl) -> Result<()> {
    if token_file.exists() {
        bail!("File '{}' already exists.", token_file.display());
    }
    let data = serde_json::to_string(token)?;
    fs::write(token_file, data).map_err(|e| anyhow!("Error writing to file: {e}"))
}

pub async fn handle_auth_file(token_file: &std::path::Path, input: &AuthFileParams) -> Result<()> {
    let secrets_file = fs::canonicalize(&input.secrets_file)
        .map_err(|e| anyhow!("File '{}' cannot be used: {e}", input.secrets_file.display()))?;

    let app = read_from_file(&secrets_file)?;
    let token = get_user_token(app).await?;
    write_to_file(token_file, &token)
}

pub async fn handle_auth(token_file: &std::path::Path, _output: &PrintVariant, input: &AuthParams) -> Result<()> {
    let client_secret = if input.client_secret_stdin {
        let mut user_input = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut user_input).expect("Error reading from stdin");
        user_input.trim().into()
    } else {
        input.client_secret.as_ref().unwrap().clone()
    };
    info!(
        "Request auth for app with client_id='{}' and client_secret='****'",
        input.client_id
    );

    let app = AppClientData::new(&input.client_id, &client_secret);
    let token = get_user_token(app).await?;
    write_to_file(token_file, &token)
}
