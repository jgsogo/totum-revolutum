use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;
use futures::StreamExt;
use tracing::debug;

use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::handy::{GetCreateFolderIfNotExistsAll, GetFolderID};
use pcloud_sdk::methods::file::uploadfile::{PostUploadFile, UploadFileParams};
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::progress_bar::ProgressBarBuilder;
use pcloud_sdk::types::FileID;
use pcloud_sdk::types::{Folder, FolderID};
use utils::filesystem::{current_path, normalize_path, to_absolute_path};

use crate::output::{progressbar_for_progresshash, Print, PrintVariant};
use crate::utils::params_or_stdin::ParamsOrStdin;
use crate::CliParams;

#[derive(Args, Debug)]
pub struct Params {
    #[clap(num_args = 0.., value_delimiter = ' ')]
    files: Vec<Utf8PathBuf>,

    /// Output folder to upload the files to.
    #[clap(long)]
    remote_dir: String,

    /// If active, files will be uploaded to nested folders inside `remote_dir` based on the
    /// relative path to current working directory.
    #[clap(long, action, default_value_t = false)]
    keep_relative_paths: bool,
}

fn remote_path_from_local_file(abs_file_to_upload: &Utf8Path, keep_relative_paths: bool) -> Result<Utf8PathBuf> {
    if keep_relative_paths {
        // Compute relative path from current working dir
        let cwd = current_path();
        let rel_file_path = abs_file_to_upload.strip_prefix(cwd.clone()).map_err(|_| {
            anyhow!(
                "File '{abs_file_to_upload}' is not within the current working dir '{cwd}'. Cannot compute relative path."
            )
        })?;
        Ok(rel_file_path.to_path_buf())
    } else {
        let filename = abs_file_to_upload.file_name().unwrap();
        Ok(Utf8PathBuf::from(filename))
    }
}

async fn upload(
    pcloud: PCloudClientImpl<OAuth2TokenImpl>,
    output: &PrintVariant,
    file_to_upload: Utf8PathBuf,
    folder: &FolderID,
    keep_relative_paths: bool,
) -> Result<FileID> {
    debug!("Upload file '{file_to_upload}' to folder {folder} (relative paths: {keep_relative_paths})");
    let abs_file_to_upload = {
        let abs_file_to_upload = normalize_path(to_absolute_path(&file_to_upload));
        if !abs_file_to_upload.exists() {
            bail!("File to upload ('{abs_file_to_upload}') doesn't exist!")
        }
        abs_file_to_upload
    };

    let (remote_folder_path, remote_filename) = {
        let remote_path_filename = remote_path_from_local_file(&abs_file_to_upload, keep_relative_paths)?;
        let remote_path = remote_path_filename
            .parent()
            .ok_or(anyhow!("Cannot get remote path from {remote_path_filename}"))?;
        let remote_filename = remote_path_filename
            .file_name()
            .ok_or(anyhow!("Cannot get remote filename from {remote_path_filename}"))?;
        debug!("Upload file '{abs_file_to_upload}' to remote at '{remote_path}'");
        (remote_path.to_path_buf(), remote_filename.to_string())
    };

    // Create folder if it doesn't exist
    let remote_folder_id = pcloud.createfolderifnotexists_all(folder, remote_folder_path).await?;

    // Get all the inputs we need for the operation
    let mut upload_params = UploadFileParams::new(remote_folder_id, remote_filename.to_string());
    let progresshash = {
        let mut s = DefaultHasher::new();
        file_to_upload.to_string().hash(&mut s);
        s.finish().to_string()
    };
    upload_params.progresshash = Some(progresshash.clone());

    let (tx, rx) = tokio::sync::oneshot::channel();
    {
        let pb_points = std::fs::metadata(abs_file_to_upload.clone())?.len();
        let pb = output.build(pb_points);
        pb.set_message(&format!("Uploading '{abs_file_to_upload}'"));
        let pcloud = pcloud.clone();
        tokio::spawn(async move {
            progressbar_for_progresshash(&pcloud, progresshash, rx, pb, pb_points).await;
        });
    }

    let r = pcloud.uploadfile(abs_file_to_upload.as_ref(), upload_params).await?;
    let _ = tx.send(());

    let file_id = r.fileids.first().unwrap();
    Ok(FileID::new(*file_id))
}

pub async fn handle(
    pcloud: PCloudClientImpl<OAuth2TokenImpl>,
    output: &PrintVariant,
    params: Params,
    cli_params: CliParams,
) -> Result<()> {
    let remote_folder = Folder::from_str(&params.remote_dir)?;
    let remote_folder = match remote_folder {
        Folder::FolderID(f) => f,
        Folder::RemotePath(p) => pcloud.get_folderid(&p).await?,
    };
    let input = ParamsOrStdin::new(params.files);

    // Execute concurrently the upload function
    debug!("Run uploads concurrently ({})", cli_params.parallel);
    let uploads = futures::stream::iter(
        input
            .into_iter()
            .map(|path| upload(pcloud.clone(), output, path, &remote_folder, params.keep_relative_paths)),
    )
    .buffer_unordered(cli_params.parallel)
    .map(|r| match r {
        Ok(file) => output.fileid(file),
        Err(e) => output.eprintln(&format!("Error uploading {e}")),
    })
    .collect::<Vec<_>>();
    uploads.await;

    Ok(())
}
