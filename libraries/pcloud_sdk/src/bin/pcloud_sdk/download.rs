use std::str::FromStr;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;
use futures::StreamExt;
use tracing::debug;

use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::handy::GetFileLinkAndDownload;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::methods::streaming::getfilelink::GetFileLinkInput;
use pcloud_sdk::types::RemotePath;
use utils::filesystem::{current_path, to_absolute_path};

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

/// Computes a local path (always inside `output_folder`) using `input` as a relative path to
/// concatenate to `output_folder`.
fn local_path_from_remote_file(output_folder: &Utf8Path, input: &RemotePath) -> Result<Utf8PathBuf> {
    let rel_path = input.as_path().strip_prefix("/")?;
    let abs_path = to_absolute_path(output_folder.join(rel_path));
    Ok(abs_path)
}

async fn download(
    pcloud: PCloudClientImpl<OAuth2TokenImpl>,
    output: &PrintVariant,
    output_folder: &Utf8Path,
    path: String,
) -> Result<Utf8PathBuf> {
    let input = RemotePath::from_str(&path)?;
    let output_path = local_path_from_remote_file(output_folder, &input)?;
    debug!("Download file '{input}' to '{output_path}'");
    let file_link = GetFileLinkInput::new(input.try_into()?);
    pcloud.getfilelink_and_download(file_link, &output_path, output).await?;
    Ok(output_path)
}

pub async fn handle(
    pcloud: PCloudClientImpl<OAuth2TokenImpl>,
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
        Err(e) => output.eprintln(&format!("Error downloading {e}")),
    })
    .collect::<Vec<_>>();
    downloads.await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_path_from_remote_file() {
        let output_path = to_absolute_path(&Utf8PathBuf::from("output_path"));

        assert_eq!(
            local_path_from_remote_file(&output_path, &RemotePath::from_str("path:/input.txt").unwrap()).unwrap(),
            output_path.join("input.txt")
        );
        // Using absolute paths, they turn to relative ones
        assert_eq!(
            local_path_from_remote_file(
                &output_path,
                &RemotePath::from_str("path:/a/path/to/input.txt").unwrap()
            )
            .unwrap(),
            output_path.join("a/path/to/input.txt")
        );
        assert_eq!(
            local_path_from_remote_file(
                &output_path,
                &RemotePath::from_str("path:/a/path/to/../other/input.txt").unwrap()
            )
            .unwrap(),
            output_path.join("a/path/other/input.txt")
        );
        // Using relative paths
        assert_eq!(
            local_path_from_remote_file(
                &output_path,
                &RemotePath::from_str("path:/a/path/to/../other/input.txt").unwrap()
            )
            .unwrap(),
            output_path.join("a/path/other/input.txt")
        );
    }
}
