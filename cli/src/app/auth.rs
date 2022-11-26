use std::path::Path;

use anyhow::Result;
use clap::Args;
use tracing::info;

use pcloud_sdk::data;
use pcloud_sync::storage;
use pcloud_sync::storage::apps::Apps;
use pcloud_sync::utils::mut_find_or_insert;

// TODO: Args 'client_secret' and 'client_secret_stdin' are mutually exclusive, but one of them is always required

#[derive(Args, Debug)]
pub struct AuthParams {
    #[clap(long)]
    /// Name to identify this application ()
    name: Option<String>,

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

pub async fn handle(home: &Path, params: &AuthParams) -> Result<()> {
    let secret = if params.client_secret_stdin {
        let mut user_input = String::new();
        let stdin = std::io::stdin(); // We get `Stdin` here.
        stdin
            .read_line(&mut user_input)
            .expect("Error reading from stdin");
        user_input.trim().into()
    } else {
        params.client_secret.as_ref().unwrap().clone()
    };
    info!(
        "Request auth for app with client_id='{}' and client_secret='{}'",
        params.client_id, secret
    );

    // Lock the file
    let path = storage::apps::AppsFile::path(home);
    let mut file_data = storage::apps::AppsFile::update_or_create(&path, Apps::default())?;

    // Search of create new entry for this application
    let app = data::app::App::default(&params.client_id, &secret);
    let (app, _inserted) = file_data.content.find_or_insert(&params.client_id, app);
    if app.client_secret != secret {
        eprintln!("Application with the same client_id but different client_secret already exists! Please, remove it first");
        std::process::exit(1);
    }

    // Run oauth request
    // TODO: Move this to SDK
    let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let app_client_data =
        data::app_client_data::AppClientData::new(&app.client_id, &app.client_secret);
    let pcloud = pcloud_sdk::client::HttpClient::authorize(app_client_data, addr)
        .await
        .expect("TODO: Propagate errors");
    let token = pcloud.oauth2_token;

    println!("Oauth2 token: {:?}", token);
    // Update application data and store new token (override if existing)
    if params.name.is_some() {
        app.name = params.name.as_ref().unwrap().into();
    }

    // TODO: Don't like repeating variables here, rustify this piece of code!
    // TODO: Move it to App struct impl
    let mytoken = data::oauth2token::OAuth2Token {
        userid: token.userid,
        locationid: token.locationid,
        access_token: token.access_token.clone(),
        token_type: token.token_type.clone(),
    };
    let (t, _inserted) = mut_find_or_insert(&mut app.tokens, |t| t.userid == token.userid, mytoken);
    if !_inserted {
        t.locationid = token.locationid;
        t.access_token = token.access_token;
        t.token_type = token.token_type;
    }
    Ok(())
}
