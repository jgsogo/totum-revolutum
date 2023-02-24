use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Args;
use tracing::debug;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::types::Folder;

use crate::output::{Print, PrintVariant};

#[derive(Args, Debug)]
pub struct Params {
    #[clap(num_args = 0.., value_delimiter = ' ')]
    folder: Vec<String>,
}

pub async fn handle(pcloud: HttpClient<OAuth2TokenImpl>, output: &PrintVariant, params: &Params) -> Result<()> {
    for it in params.folder.iter() {
        let folder = Folder::from_str(&it).map_err(|e| anyhow!("Cannot parse input parameter 'folder': {e}"))?;
        debug!("Listfolder {it:?}");
        let input = ListFolderInput::new(folder);
        let l = pcloud.listfolder(input).await?;
        output.list_folder(&l);
    }
    Ok(())
}
