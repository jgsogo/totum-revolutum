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
use crate::stdin_lines::StdinLines;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(num_args = 0.., value_delimiter = ' ')]
    folders: Vec<String>,
}

enum ParamsOrStdin {
    Params(std::vec::IntoIter<String>),
    Stdin(StdinLines),
}

impl ParamsOrStdin {
    pub fn new(folders: Vec<String>) -> Self {
        if folders.is_empty() {
            debug!("No folders provided, will iterate from stdin");
            ParamsOrStdin::Stdin(StdinLines {})
        } else {
            ParamsOrStdin::Params(folders.into_iter())
        }
    }
}

impl<'a> Iterator for ParamsOrStdin {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ParamsOrStdin::Params(p) => p.next(),
            ParamsOrStdin::Stdin(s) => s.next(),
        }
    }
}

pub async fn handle(pcloud: HttpClient<OAuth2TokenImpl>, output: &PrintVariant, params: Params) -> Result<()> {
    let input = ParamsOrStdin::new(params.folders);
    for it in input {
        let folder = Folder::from_str(&it).map_err(|e| anyhow!("Cannot parse input parameter 'folder': {e}"))?;
        debug!("Listfolder {it}");
        let input = ListFolderInput::new(folder);
        let l = pcloud.listfolder(input).await?;
        output.list_folder(&l)?;
    }
    Ok(())
}
