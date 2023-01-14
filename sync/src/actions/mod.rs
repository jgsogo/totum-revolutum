use std::time::Instant;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::actions::action_run::ActionRun;
use crate::diff;
use crate::diff::FilePair;
use crate::filesystem::{FileMetadata, Filesystem};
use crate::storage::config;

mod action_run;
mod backup;

/// Describes the action to perform
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, clap::ValueEnum, Copy)]
pub enum Actions {
    /// Send local content to remote. Local is not modified
    Backup,

    /// Send local content to remote in a zip file.
    ZipBackup,

    /// Keep local and remote synced.
    Sync,

    /// Send remote content to local folder. Remote is not modified
    Dump,

    /// Move content from local to remote. Local will be removed
    MoveUpload,

    /// Move content from remote to local. Remote will be removed
    MoveDownload,
}

/// Describe the action to take when there are conflicts. Not all
/// [`OnConflict`] are compatible with every [`Actions`]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, clap::ValueEnum, Copy)]
pub enum OnConflict {
    /// Overrides remote content with local
    OverrideRemote,

    /// Overrides local content with remote
    OverrideLocal,

    /// Renames remote content before copying local one
    RenameRemote,

    /// Renames local content before copying remote one
    RenameLocal,

    /// Overrides with new content according to _modification time_
    KeepLatest,
}

async fn work_on_results<FsLhsMetadata: FileMetadata + 'static, FsRhsMetadata: FileMetadata + 'static>(
    rx: flume::Receiver<FilePair<FsLhsMetadata, FsRhsMetadata>>,
    action: &dyn ActionRun<FsLhsMetadata, FsRhsMetadata>,
) -> Result<()> {
    info!("Start backup receiving loop");
    let start = Instant::now();
    while let Ok(file_pair) = rx.recv_async().await {
        // TODO: We have independent actions here that can run in parallel!
        action.run(file_pair).await?;
    }
    info!("Finished backup receiving loop in {:?}", start.elapsed());
    Ok(())
}

pub async fn run<FsLhs: Filesystem + 'static, FsRhs: Filesystem + 'static>(
    lhs_fs: FsLhs,
    rhs_fs: FsRhs,
    config: &config::Config,
) -> Result<()> {
    let action_run = match config.action.action() {
        Actions::Backup => backup::Backup::new(&lhs_fs, &rhs_fs, *config.action.conflict()),
        Actions::ZipBackup => todo!("impl pending"),
        Actions::Sync => todo!("impl pending"),
        Actions::Dump => todo!("impl pending"),
        Actions::MoveUpload => todo!("impl pending"),
        Actions::MoveDownload => todo!("impl pending"),
    };

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) = diff::two_ways_run::<FsLhs::Metadata, FsRhs::Metadata>().await;
    if let Err(e) = tokio::try_join!(
        lhs_fs.walk_directory(lhs, 6),
        rhs_fs.walk_directory(rhs, 6),
        work_on_results(differ, &action_run),
    ) {
        error!("Error on workers loop: {e}");
    }

    action_run.stats();
    Ok(())
}
