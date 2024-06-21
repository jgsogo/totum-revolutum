use anyhow::Result;

use filesystem::FilesystemOps;
use serde::{Deserialize, Serialize};

use crate::storage::config;

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

pub async fn run<FsLhs: FilesystemOps, FsRhs: FilesystemOps>(
    lhs_fs: FsLhs,
    mut rhs_fs: FsRhs,
    config: &config::Config,
) -> Result<()> {
    match config.action.action() {
        Actions::Backup => {
            let backup_conflict = match config.action.conflict() {
                OnConflict::OverrideRemote => filesystem::diff::impls::BackupConflict::Override,
                OnConflict::OverrideLocal => todo!("not impl"),
                OnConflict::RenameRemote => todo!("not impl"),
                OnConflict::RenameLocal => todo!("not impl"),
                OnConflict::KeepLatest => todo!("not impl"),
            };
            filesystem::diff::impls::backup(&lhs_fs, &mut rhs_fs, backup_conflict).await?;
            Ok(())
        }
        Actions::ZipBackup => todo!("impl pending"),
        Actions::Sync => todo!("impl pending"),
        Actions::Dump => todo!("impl pending"),
        Actions::MoveUpload => todo!("impl pending"),
        Actions::MoveDownload => todo!("impl pending"),
    }
}
