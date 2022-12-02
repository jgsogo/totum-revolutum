use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::diff::{FileMetadata, Filesystem};

pub mod backup;

// #[async_trait]
// pub trait Copy<LHS: FileMetadata, RHS: FileMetadata>: Filesystem<RHS> {
//     async fn copy(&self, lhs: &LHS, rhs: Option<RHS>) -> Result<(&LHS, RHS)>;
// }
//
// #[async_trait]
// pub trait Remove<T: FileMetadata>: Filesystem<T> {
//     async fn remove(&self, file: T) -> Result<()>;
// }
//
// // #[async_trait]
// // pub trait Move<LHS: FileMetadata, RHS: FileMetadata>: BasePoint<LHS> + Copy<LHS, RHS> /*+ Remove<LHS>*/ {
// //     async fn do_move(&self, lhs: LHS, rhs: Option<RHS>) -> Result<RHS> {
// //         let (_, rhs) = self.copy(&lhs, rhs).await?;
// //         self.remove(lhs).await?; // TODO: I can't do this here, it doesn't belong to this BasePoint
// //         Ok(rhs)
// //     }
// // }
//
// #[async_trait]
// pub trait Rename<T: FileMetadata>: Filesystem<T> {
//     async fn rename(&self, file: T) -> Result<T>;
// }

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
