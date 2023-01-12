use std::path::Path;
use std::time::Instant;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
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

struct Backup<'action, FsLhs: Filesystem, FsRhs: Filesystem> {
    lhs_fs: &'action FsLhs,
    rhs_fs: &'action FsRhs,
    on_conflict: OnConflict,
}

impl<'action, FsLhs: Filesystem, FsRhs: Filesystem> Backup<'action, FsLhs, FsRhs> {
    pub fn new(lhs_fs: &'action FsLhs, rhs_fs: &'action FsRhs, on_conflict: OnConflict) -> Self {
        Self {
            lhs_fs,
            rhs_fs,
            on_conflict,
        }
    }
}

#[async_trait]
trait ActionRun<FsLhsMetadata: FileMetadata + 'static, FsRhsMetadata: FileMetadata + 'static>: Sync {
    async fn run(&self, file_pair: FilePair<FsLhsMetadata, FsRhsMetadata>) -> Result<()> {
        match file_pair {
            // Both files exist
            FilePair {
                lhs: Some(lhs),
                rhs: Some(rhs),
            } => {
                if lhs.eq(&rhs) {
                    self.run_with_both_eq(&lhs, &rhs).await
                } else {
                    self.run_with_both(&lhs, &rhs).await
                }
            }
            // Only LHS exist
            FilePair { lhs: Some(lhs), .. } => self.run_with_lhs(&lhs).await,
            // Only RHS exist
            FilePair { rhs: Some(rhs), .. } => self.run_with_rhs(&rhs).await,
            // None exist
            _ => Err(anyhow!("None sides of the file exist, unexpected error!")),
        }
    }

    async fn run_with_both_eq(&self, _lhs: &FsLhsMetadata, _rhs: &FsRhsMetadata) -> Result<()> {
        Ok(())
    }

    async fn run_with_both(&self, _lhs: &FsLhsMetadata, _rhs: &FsRhsMetadata) -> Result<()> {
        Ok(())
    }

    async fn run_with_lhs(&self, _lhs: &FsLhsMetadata) -> Result<()> {
        Ok(())
    }

    async fn run_with_rhs(&self, _rhs: &FsRhsMetadata) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl<'action, FsLhs: Filesystem + 'static, FsRhs: Filesystem + 'static> ActionRun<FsLhs::Metadata, FsRhs::Metadata>
    for Backup<'action, FsLhs, FsRhs>
{
    async fn run_with_both(&self, lhs: &FsLhs::Metadata, rhs: &FsRhs::Metadata) -> Result<()> {
        match self.on_conflict {
            OnConflict::OverrideRemote => {
                info!("Override remote '{}'", lhs.id());
                // let _r = remote_basepoint.copy(&lhs, Some(rhs)).await?;
            }
            OnConflict::RenameRemote => {
                info!("Rename remote and copy '{}'", rhs.id());
                // let _ = remote_basepoint.rename(rhs).await?;
                // let _r = remote_basepoint.copy(&lhs, None).await?;
            }
            s => panic!("Not a valid onConflict for backup: {s:?}"),
        }
        Ok(())
    }

    async fn run_with_lhs(&self, lhs: &FsLhs::Metadata) -> Result<()> {
        info!("Copy to remote '{}'", lhs.id());
        Ok(())
    }
}

async fn work_on_results<FsLhsMetadata: FileMetadata + 'static, FsRhsMetadata: FileMetadata + 'static>(
    rx: flume::Receiver<FilePair<FsLhsMetadata, FsRhsMetadata>>,
    action: &dyn ActionRun<FsLhsMetadata, FsRhsMetadata>,
) -> Result<()> {
    info!("Start backup receiving loop");
    let start = Instant::now();
    while let Ok(file_pair) = rx.recv_async().await {
        // TODO: We have independent actions here that can be parallelized
        action.run(file_pair).await?;
    }
    info!("Finished backup receiving loop in {:?}", start.elapsed());
    Ok(())
}

pub async fn run(home: &Path, path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    let local_fs = FilesystemLocal::new(path)?;

    let remote_fs = {
        let pcloud = config.auth.get_pcloud_client(home)?;
        let base_path = config.auth.remote_path.as_ref().unwrap_or(&"/".to_string()).clone();
        FilesystemPCloud::new(Path::new(&base_path), pcloud.clone()).await?
    };

    let backup_action = Backup::new(&local_fs, &remote_fs, config.action.conflict().clone());

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) = diff::two_ways_run::<local::LocalMetadata, remote::RemoteMetadata>().await;
    if let Err(e) = tokio::try_join!(
        local_fs.walk_directory(lhs, 6),
        remote_fs.walk_directory(rhs, 6),
        work_on_results(differ, &backup_action),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}
