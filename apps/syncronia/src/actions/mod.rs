use anyhow::Result;
use filesystem::diff::two_way_diff;
use filesystem::Filesystem;
use serde::{Deserialize, Serialize};

use crate::storage::config;

mod backup;

/// Describes the action to perform
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
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

pub async fn run<FsLhs: Filesystem + 'static, FsRhs: Filesystem + 'static>(
    lhs_fs: FsLhs,
    rhs_fs: FsRhs,
    config: &config::Config,
) -> Result<()> {
    let mut action_run = match config.action.action() {
        Actions::Backup => backup::Backup::new(&lhs_fs, &rhs_fs, *config.action.conflict()),
        Actions::ZipBackup => todo!("impl pending"),
        Actions::Sync => todo!("impl pending"),
        Actions::Dump => todo!("impl pending"),
        Actions::MoveUpload => todo!("impl pending"),
        Actions::MoveDownload => todo!("impl pending"),
    };

    two_way_diff::run(&lhs_fs, &rhs_fs, &mut action_run).await?;

    // action_run.stats();
    Ok(())
}
