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
use crate::utils::params_or_stdin::ParamsOrStdin;
use crate::CliParams;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(num_args = 0.., value_delimiter = ' ')]
    files: Vec<String>,

    /// Output folder to download the files to. Defaults to current path.
    #[clap(long, default_value_t=current_path())]
    output_dir: Utf8PathBuf,
}

async fn download(
    pcloud: HttpClient<OAuth2TokenImpl>,
    output: &PrintVariant,
    output_folder: &Utf8Path,
    input: String,
) -> Result<Utf8PathBuf> {
    let file = File::from_str(&input).map_err(|e| anyhow!("Cannot parse input parameter 'file': {e}"))?;
    let output_path = {
        let with_current_path = output_folder.join(Utf8Path::new(&input).strip_prefix("/")?); // TODO: Here we need some tests. I'm assuming that `File::from_str` has already validated that it is an absolute path
        to_absolute_path(&with_current_path)
    };
    debug!("Download file '{input}' to '{output_path}'");
    let file_link = GetFileLinkInput::new(file);
    pcloud.getfilelink_and_download(file_link, &output_path, output).await?;
    Ok(output_path)
}

pub async fn handle(
    pcloud: HttpClient<OAuth2TokenImpl>,
    output: &PrintVariant,
    params: Params,
    cli_params: CliParams,
) -> Result<()> {
    let output_folder = params.output_dir;
    let input = ParamsOrStdin::new(params.files);

    // Execute concurrently the download function
    let downloads = futures::stream::iter(
        input
            .into_iter()
            .map(|path| download(pcloud.clone(), output, &output_folder, path)),
    )
    .buffer_unordered(cli_params.parallel)
    .map(|r| match r {
        Ok(file) => output.path(file),
        Err(e) => output.eprintln(&*format!("Error downloading {e}")),
    })
    .collect::<Vec<_>>();
    downloads.await;

    Ok(())
}
