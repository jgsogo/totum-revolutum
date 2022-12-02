use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};

use crate::actions::{Copy, OnConflict, Rename};
use crate::diff;
use crate::diff::filesystem::{FileMetadata, Filesystem};
use crate::diff::two_ways::FileDiff;
use crate::local;
use crate::local::FilesystemLocal;
use crate::remote;
use crate::remote::filesystem::FilesystemPCloud;
use crate::storage::config;

async fn backup<LHS, RHS, BasePointLHS, BasePointRHS>(
    filediff: FileDiff<LHS, RHS>,
    config: &config::Config,
    _local_basepoint: &BasePointLHS, // &dyn BasePoint<LHS>,
    remote_basepoint: &BasePointRHS, //&dyn BasePoint<RHS>,
) -> Result<()>
where
    LHS: FileMetadata,
    RHS: FileMetadata,
    BasePointLHS: Filesystem<LHS>,
    BasePointRHS: Filesystem<RHS> + Copy<LHS, RHS> + Rename<RHS>,
{
    match filediff {
        FileDiff {
            lhs: Some(lhs),
            rhs: Some(rhs),
        } => match config.action.conflict() {
            OnConflict::OverrideRemote => {
                let _r = remote_basepoint.copy(&lhs, Some(rhs)).await?;
                Ok(())
            }
            OnConflict::RenameRemote => {
                let _ = remote_basepoint.rename(rhs).await?;
                let _r = remote_basepoint.copy(&lhs, None).await?;
                Ok(())
            }
            s => panic!("Not a valid onConflict for backup: {s:?}"),
        },
        FileDiff { lhs: Some(lhs), .. } => {
            let _r = remote_basepoint.copy(&lhs, None).await?;
            Ok(())
        }
        FileDiff { rhs: Some(_), .. } => Ok(()),
        _ => panic!("Not expected"),
    }
}

async fn work_on_results<LHS, RHS, BasePointLHS, BasePointRHS>(
    rx: flume::Receiver<FileDiff<LHS, RHS>>,
    config: &config::Config,
    local_basepoint: &BasePointLHS,
    remote_basepoint: &BasePointRHS,
) -> Result<()>
where
    LHS: FileMetadata,
    RHS: FileMetadata,
    BasePointLHS: Filesystem<LHS>,
    BasePointRHS: Filesystem<RHS> + Copy<LHS, RHS> + Rename<RHS>,
{
    info!("Start backup receiving loop");
    let start = Instant::now();
    while let Ok(v) = rx.recv_async().await {
        // TODO: We have independent actions here that can be parallelized
        backup(v, config, local_basepoint, remote_basepoint).await?;
    }
    info!("Finished backup receiving loop in {:?}", start.elapsed());
    Ok(())
}

pub async fn run(home: &Path, path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) = diff::two_ways::run::<local::LocalMetadata, remote::RemoteMetadata>().await;

    let local_basepoint = FilesystemLocal::new(path, lhs);

    let pcloud = config.auth.get_pcloud_client(home)?;
    let remote_basepoint = FilesystemPCloud::new(pcloud.clone(), rhs);

    if let Err(e) = tokio::try_join!(
        local_basepoint.walk_directory(6),
        remote_basepoint.walk_directory(6, config),
        work_on_results(differ, config, &local_basepoint, &remote_basepoint),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}
