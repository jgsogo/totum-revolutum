use std::str::FromStr;

use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;
use futures::StreamExt;
use tracing::debug;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::handy::GetFileLinkAndDownload;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::methods::streaming::getfilelink::GetFileLinkInput;
use pcloud_sdk::types::File;
use pcloud_sdk::utils::{current_path, to_absolute_path};

use crate::output::{Print, PrintVariant};
use crate::stdin_lines::StdinLines;
use crate::CliParams;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(num_args = 0.., value_delimiter = ' ')]
    files: Vec<String>,
}

// TODO: Probably more complex than needed...
enum ParamsOrStdin {
    Params(std::vec::IntoIter<String>),
    Stdin(StdinLines),
}

impl ParamsOrStdin {
    pub fn new(files: Vec<String>) -> Self {
        if files.is_empty() {
            debug!("No files provided, will iterate from stdin");
            ParamsOrStdin::Stdin(StdinLines {})
        } else {
            ParamsOrStdin::Params(files.into_iter())
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

async fn download(pcloud: HttpClient<OAuth2TokenImpl>, output: &PrintVariant, input: String) -> Result<Utf8PathBuf> {
    let file = File::from_str(&input).map_err(|e| anyhow!("Cannot parse input parameter 'file': {e}"))?;
    let output_path = {
        let with_current_path = current_path().join(Utf8Path::new(&input).strip_prefix("/")?); // TODO: Here we need some tests. I'm assuming that `File::from_str` has already validated that it is an absolute path
        to_absolute_path(&with_current_path)
    };
    debug!("Download file '{input}' to '{output_path}'");
    let fileLink = GetFileLinkInput::new(file);
    pcloud.getfilelink_and_download(fileLink, &output_path, output).await?;
    Ok(output_path)
}

pub async fn handle(
    pcloud: HttpClient<OAuth2TokenImpl>,
    output: &PrintVariant,
    params: Params,
    cli_params: CliParams,
) -> Result<()> {
    let input = ParamsOrStdin::new(params.files);

    // Execute concurrently the download function
    let downloads = futures::stream::iter(input.into_iter().map(|path| download(pcloud.clone(), output, path)))
        .buffer_unordered(cli_params.parallel)
        .map(|r| match r {
            Ok(file) => output.path(file),
            Err(e) => output.eprintln(&*format!("Error downloading {e}")),
        })
        .collect::<Vec<_>>();
    downloads.await;

    Ok(())
}
