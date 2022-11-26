use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{debug, error, info, trace};

use crate::actions::OnConflict;
use crate::diff;
use crate::diff::basepoint::FileMetadata;
use crate::diff::two_ways::FileDiff;
use crate::local;
use crate::remote;
use crate::storage::config;

struct Actions<'a, LHSMetadata: FileMetadata, RHSMetadata: FileMetadata> {
    file_diff: FileDiff<LHSMetadata, RHSMetadata>,
    config: &'a config::Config,
    pcloud: pcloud_sdk::client::HttpClient,
}

impl<'a, LHSMetadata: FileMetadata, RHSMetadata: FileMetadata>
    Actions<'a, LHSMetadata, RHSMetadata>
{
    pub fn new(
        file_diff: FileDiff<LHSMetadata, RHSMetadata>,
        config: &'a config::Config,
        pcloud: pcloud_sdk::client::HttpClient,
    ) -> Self {
        Self {
            file_diff,
            config,
            pcloud,
        }
    }

    async fn copy_to_lhs(&self) -> Result<()> {
        trace!("copy_to_lhs({})", self.file_diff.id());
        // TODO: to implement
        Ok(())
    }
    async fn copy_to_rhs(&self) -> Result<()> {
        trace!("copy_to_rhs({})", self.file_diff.id());
        // TODO: to implement
        Ok(())
    }
    async fn rename_lhs(&self) -> Result<()> {
        trace!("rename_lhs({})", self.file_diff.id());
        // TODO: to implement
        Ok(())
    }
    async fn rename_rhs(&self) -> Result<()> {
        trace!("rename_rhs({})", self.file_diff.id());
        // TODO: to implement
        Ok(())
    }
}

impl<'a, LHSMetadata: FileMetadata, RHSMetadata: FileMetadata>
    Actions<'a, LHSMetadata, RHSMetadata>
{
    pub async fn backup(&self) -> Result<()> {
        match &self.file_diff {
            FileDiff {
                lhs_metadata: Some(lhs_metadata),
                rhs_metadata: Some(rhs_metadata),
            } => {
                if lhs_metadata.eq(rhs_metadata) {
                    Ok(())
                } else {
                    self.backup_modified().await
                }
            }
            FileDiff {
                lhs_metadata: Some(_),
                ..
            } => self.copy_to_rhs().await,
            FileDiff {
                rhs_metadata: Some(_),
                ..
            } => Ok(()),
            _ => panic!("Not expected"),
        }
    }

    async fn backup_modified(&self) -> Result<()> {
        match self.config.action.conflict() {
            OnConflict::OverrideRemote => self.copy_to_rhs().await,
            OnConflict::RenameRemote => {
                self.rename_rhs().await?;
                self.copy_to_rhs().await
            }
            s => panic!("Not a valid onConflict for backup: {s:?}"),
        }
    }
}

async fn work_on_results<
    LHSMetadata: FileMetadata + 'static,
    RHSMetadata: FileMetadata + 'static,
>(
    rx: flume::Receiver<FileDiff<LHSMetadata, RHSMetadata>>,
    config: &config::Config,
    pcloud: pcloud_sdk::client::HttpClient,
) -> Result<()> {
    info!("Start backup receiving loop");
    let start = Instant::now();
    while let Ok(v) = rx.recv_async().await {
        // TODO: We have independent actions here that can be parallelized
        Actions::new(v, &config, pcloud.clone()).backup().await?;
    }
    info!("Finished backup receiving loop in {:?}", start.elapsed());
    Ok(())
}

pub async fn run(home: &Path, path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());
    let pcloud = config.auth.get_pcloud_client(home)?;

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) =
        diff::two_ways::run::<local::LocalMetadata, remote::RemoteMetadata>().await;

    if let Err(e) = tokio::try_join!(
        local::walk_local_directory(path, 6, lhs),
        remote::walk_remote_directory(config, 6, rhs, pcloud.clone()),
        work_on_results(differ, config, pcloud),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}
