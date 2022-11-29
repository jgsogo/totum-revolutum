use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{error, info};

use crate::actions::OnConflict;
use crate::diff::basepoint::{BasePointDiffImpl, FileMetadata};
use crate::diff::two_ways::FileDiff;
use crate::local;
use crate::local::BasePointLocal;
use crate::remote;
use crate::remote::basepoint::BasePointPCloud;
use crate::storage::config;
use crate::{actions, diff};

async fn backup<LHSMetadata, RHSMetadata>(
    filediff: FileDiff<LHSMetadata, RHSMetadata>,
    config: &config::Config,
    local_basepoint: &BasePointLocal,
    remote_basepoint: &BasePointPCloud,
) -> Result<()>
where
    LHSMetadata: FileMetadata + actions::Copy<RHSMetadata>,
    RHSMetadata: FileMetadata + actions::Rename,
{
    match filediff {
        FileDiff {
            lhs: Some(lhs_metadata),
            rhs: Some(rhs_metadata),
        } => match config.action.conflict() {
            OnConflict::OverrideRemote => lhs_metadata.copy(Some(rhs_metadata)).await.map(|_| ()),
            OnConflict::RenameRemote => {
                rhs_metadata.rename().await.map(|_| ())?;
                lhs_metadata.copy(None).await.map(|_| ())
            }
            s => panic!("Not a valid onConflict for backup: {s:?}"),
        },
        FileDiff {
            lhs: Some(lhs_metadata),
            ..
        } => {
            let id = lhs_metadata.id().to_string();
            let _ = lhs_metadata.copy(None).await?;
            Ok(())
        }
        FileDiff { rhs: Some(_), .. } => Ok(()),
        _ => panic!("Not expected"),
    }
}

async fn work_on_results<
    LHSMetadata: FileMetadata + actions::Copy<RHSMetadata> + 'static,
    RHSMetadata: FileMetadata + actions::Rename + 'static,
>(
    rx: flume::Receiver<FileDiff<LHSMetadata, RHSMetadata>>,
    config: &config::Config,
    local_basepoint: &BasePointLocal,
    remote_basepoint: &BasePointPCloud,
) -> Result<()> {
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

    let local_basepoint = BasePointLocal::new(path, lhs);

    let pcloud = config.auth.get_pcloud_client(home)?;
    let remote_basepoint = BasePointPCloud::new(pcloud.clone(), rhs);

    if let Err(e) = tokio::try_join!(
        local_basepoint.walk_local_directory(6),
        remote_basepoint.walk_remote_directory(6, &config),
        work_on_results(differ, config, &local_basepoint, &remote_basepoint),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}
