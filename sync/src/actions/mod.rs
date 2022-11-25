use serde::{Deserialize, Serialize};

pub mod backup;

/// Describes the action to perform
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum Actions {
    /// Send local content to remote
    #[default]
    Backup,

    /// Send local content to remote in zip file
    ZipBackup,

    /// Keep local and remote synced
    Sync,

    /// Send remote content to local folder
    Dump,
}

/// Describe the action to take when there are conflicts. Not all
/// [`OnConflict`] are compatible with every [`Actions`]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum OnConflict {
    OverrideRemote,
    OverrideLocal,
    RenameRemote,
    RenameLocal,

    #[default]
    Fail,
}

/// Describe the action to take after a successful backup or dump. The
/// original file can be removed or kept.
/// TODO: This is not taken into account anywhere
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum AfterSend {
    Remove,

    #[default]
    Keep,
}
