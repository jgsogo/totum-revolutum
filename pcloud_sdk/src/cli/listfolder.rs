use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Args;
use tracing::debug;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::types::Folder;

#[derive(Args, Debug)]
pub struct Params {
    /// Output prepared to be piped to other commands
    #[clap(long, action)]
    porcelain: bool,

    // #[clap(flatten)]
    folder: String,
}

pub async fn handle(pcloud: HttpClient<OAuth2TokenImpl>, params: &Params) -> Result<()> {
    let folder = Folder::from_str(&params.folder).map_err(|_| anyhow!("Cannot parse input parameter 'folder'"))?;
    debug!("Listfolder {folder:?}");
    let input = ListFolderInput::new(folder);
    let l = pcloud.listfolder(&input).await?;
    println!("{:#?}", l);
    Ok(())
}
