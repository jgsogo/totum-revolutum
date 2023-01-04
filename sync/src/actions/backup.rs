use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};

use crate::actions::OnConflict;
use crate::diff;
use crate::diff::FileDiff;
use crate::diff::Filesystem;
use crate::local;
use crate::local::FilesystemLocal;
use crate::remote;
use crate::remote::filesystem::FilesystemPCloud;
use crate::storage::config;

async fn backup<FsLhs, FsRhs>(
    filediff: FileDiff<FsLhs::Metadata, FsRhs::Metadata>,
    config: &config::Config,
    _local_basepoint: &FsLhs,
    _remote_basepoint: &FsRhs,
) -> Result<()>
where
    FsLhs: Filesystem,
    FsRhs: Filesystem,
{
    match filediff {
        FileDiff {
            lhs: Some(_lhs),
            rhs: Some(_rhs),
        } => match config.action.conflict() {
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
        },
        FileDiff { lhs: Some(_lhs), .. } => {
            // let _r = remote_basepoint.copy(&lhs, None).await?;
            Ok(())
        }
        FileDiff { rhs: Some(_), .. } => Ok(()),
        _ => panic!("Not expected"),
    }
}

async fn work_on_results<FsLhs, FsRhs>(
    rx: flume::Receiver<FileDiff<FsLhs::Metadata, FsRhs::Metadata>>,
    config: &config::Config,
    local_basepoint: &FsLhs,
    remote_basepoint: &FsRhs,
) -> Result<()>
where
    FsLhs: Filesystem,
    FsRhs: Filesystem,
{
    info!("Start backup receiving loop");
    let start = Instant::now();
    while let Ok(v) = rx.recv_async().await {
        // TODO: We have independent actions here that can be parallelized
        info!("Backup work on {}", v.id());
        backup(v, config, local_basepoint, remote_basepoint).await?;
    }
    info!("Finished backup receiving loop in {:?}", start.elapsed());
    Ok(())
}

pub async fn run(home: &Path, path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) = diff::two_ways_run::<local::LocalMetadata, remote::RemoteMetadata>().await;

    let local_basepoint = FilesystemLocal::new(path)?;

    let pcloud = config.auth.get_pcloud_client(home)?;
    let base_path = config.auth.remote_path.as_ref().unwrap_or(&"/".to_string()).clone();
    let remote_basepoint = FilesystemPCloud::new(Path::new(&base_path), pcloud.clone()).await?;

    if let Err(e) = tokio::try_join!(
        local_basepoint.walk_directory(lhs, 6),
        remote_basepoint.walk_directory(rhs, 6),
        work_on_results(differ, config, &local_basepoint, &remote_basepoint),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}
