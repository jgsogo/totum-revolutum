use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::diff::basepoint::FileMetadata;

pub mod backup;

#[async_trait]
pub trait Copy<Rhs: FileMetadata = Self>: FileMetadata {
    async fn copy(self, rhs: Option<Rhs>) -> Result<(Self, Rhs)>;
}

#[async_trait]
pub trait Remove: FileMetadata {
    async fn remove(self) -> Result<()>;
}

#[async_trait]
pub trait Move<Rhs: FileMetadata + 'static = Self>: FileMetadata + Copy<Rhs> + Remove {
    async fn do_move(self, rhs: Option<Rhs>) -> Result<Rhs> {
        let (lhs, rhs) = self.copy(rhs).await?;
        lhs.remove().await?;
        Ok(rhs)
    }
}

#[async_trait]
pub trait Rename: FileMetadata {
    async fn rename(self) -> Result<Self>;
}

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
