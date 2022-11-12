use std::path::Path;

use clap::Args;
use pcloud_sdk_desktop::storage;
use tracing::info;

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

pub fn handle(home: &Path, params: &AuthParams) {
    let secret = if params.client_secret_stdin {
        let mut user_input = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut user_input);
        user_input
    } else {
        params.client_secret.as_ref().unwrap().clone()
    };
    info!(
        "Request auth for app with client_id='{}' and client_secret='*****'",
        params.client_id
    );

    // Lock the file
    let file_data = storage::apps::FileData::write(home);
    //if file_data.apps.

    println!("Auth application: {:?}", params);
}
