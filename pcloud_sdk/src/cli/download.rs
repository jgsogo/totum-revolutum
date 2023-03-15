use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;
use futures::StreamExt;
use tracing::debug;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::handy::GetFileLinkAndDownload;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::methods::streaming::getfilelink::GetFileLinkInput;
use pcloud_sdk::types::File;
use pcloud_sdk::utils::{current_path, normalize_path, to_absolute_path};

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
fn local_path_from_remote_file(output_folder: &Utf8Path, input: &str) -> Result<Utf8PathBuf> {
    let input = normalize_path(input);
    let input = match input.strip_prefix("/") {
        Ok(p) => p,
        Err(_) => &input,
    };

    if input.starts_with("..") {
        bail!("Local file {input} is outside the output_folder");
    }

    let abs_path = to_absolute_path(&output_folder.join(input));
    Ok(abs_path)
}

async fn download(
    pcloud: HttpClient<OAuth2TokenImpl>,
    output: &PrintVariant,
    output_folder: &Utf8Path,
    input: String,
) -> Result<Utf8PathBuf> {
    let file = File::from_str(&input).map_err(|e| anyhow!("Cannot parse input parameter 'file': {e}"))?;
    let output_path = local_path_from_remote_file(output_folder, &input)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_path_from_remote_file() {
        let output_path = to_absolute_path(&Utf8PathBuf::from("output_path"));

        assert_eq!(
            local_path_from_remote_file(&output_path, "input.txt").unwrap(),
            output_path.join("input.txt")
        );
        // Using absolute paths, they turn to relative ones
        assert_eq!(
            local_path_from_remote_file(&output_path, "/a/path/to/input.txt").unwrap(),
            output_path.join("a/path/to/input.txt")
        );
        assert_eq!(
            local_path_from_remote_file(&output_path, "/a/path/to/../other/input.txt").unwrap(),
            output_path.join("a/path/other/input.txt")
        );
        // Using relative paths
        assert_eq!(
            local_path_from_remote_file(&output_path, "a/path/to/../other/input.txt").unwrap(),
            output_path.join("a/path/other/input.txt")
        );

        // It's an error if it tries to store in parent folder
        assert!(local_path_from_remote_file(&output_path, "/a/../../leak/input.txt").is_err());
    }
}
