use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{debug, error, info};

use crate::actions::OnConflict;
use crate::diff;
use crate::diff::Filesystem;
use crate::diff::{FileMetadata, FilePair};
use crate::local;
use crate::local::FilesystemLocal;
use crate::remote;
use crate::remote::filesystem::FilesystemPCloud;
use crate::storage::config;

async fn backup<FsLhs, FsRhs>(
    file_pair: FilePair<FsLhs::Metadata, FsRhs::Metadata>,
    config: &config::Config,
    _lhs_fs: &FsLhs,
    _rhs_fs: &FsRhs,
) -> Result<()>
where
    FsLhs: Filesystem,
    FsRhs: Filesystem,
{
    match file_pair {
        FilePair {
            lhs: Some(lhs),
            rhs: Some(rhs),
        } => {
            if lhs.eq(&rhs) {
                debug!("Noting to do. Files are equal");
                Ok(())
            } else {
                match config.action.conflict() {
                    OnConflict::OverrideRemote => {
                        // let _r = remote_basepoint.copy(&lhs, Some(rhs)).await?;
                        Ok(())
                    }
                    OnConflict::RenameRemote => {
                        // let _ = remote_basepoint.rename(rhs).await?;
                        // let _r = remote_basepoint.copy(&lhs, None).await?;
                        Ok(())
                    }
                    s => panic!("Not a valid onConflict for backup: {s:?}"),
                }
            }
        }
        FilePair { lhs: Some(_lhs), .. } => {
            // let _r = remote_basepoint.copy(&lhs, None).await?;
            Ok(())
        }
        FilePair { rhs: Some(_), .. } => Ok(()),
        _ => panic!("Not expected"),
    }
}

async fn work_on_results<FsLhs: Filesystem, FsRhs: Filesystem>(
    rx: flume::Receiver<FilePair<FsLhs::Metadata, FsRhs::Metadata>>,
    config: &config::Config,
    lhs_fs: &FsLhs,
    rhs_fs: &FsRhs,
) -> Result<()> {
    info!("Start backup receiving loop");
    let start = Instant::now();
    while let Ok(v) = rx.recv_async().await {
        // TODO: We have independent actions here that can be parallelized
        info!("Backup work on {}", v.id());
        backup(v, config, lhs_fs, rhs_fs).await?;
    }
    info!("Finished backup receiving loop in {:?}", start.elapsed());
    Ok(())
}

pub async fn run(home: &Path, path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) = diff::two_ways_run::<local::LocalMetadata, remote::RemoteMetadata>().await;

    let local_fs = FilesystemLocal::new(path)?;

    let remote_fs = {
        let pcloud = config.auth.get_pcloud_client(home)?;
        let base_path = config.auth.remote_path.as_ref().unwrap_or(&"/".to_string()).clone();
        FilesystemPCloud::new(Path::new(&base_path), pcloud.clone()).await?
    };

    if let Err(e) = tokio::try_join!(
        local_fs.walk_directory(lhs, 6),
        remote_fs.walk_directory(rhs, 6),
        work_on_results(differ, config, &local_fs, &remote_fs),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}
