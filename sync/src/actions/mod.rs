use serde::{Deserialize, Serialize};

pub mod backup;

/// Describes the action to perform
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum Actions {
    /// Send local content to remote. Local is never touched and nothing will be removed from
    /// remote. It can be combined with `OnConflict::OverrideRemote` or
    /// `OnConflict::RenameRemote`.
    #[default]
    Backup,

    /// Send local content to remote in a zip file. It can only be combined with
    /// `OnConflict::OverrideRemote` or `OnConflict::RenameRemote`
    ZipBackup,

    /// Keep local and remote synced. It can be combined with:
    /// * `OnConflict::KeepLatest`: modification time will decide which file to keep
    /// * `OnConflict::OverrideLocal`: local will always override remote
    /// * `OnConflict::OverrideRemote`: remote will always override local
    Sync,

    /// Send remote content to local folder. It can be combined with
    /// `OnConflict::OverrideLocal` or `OnConflict::RenameLocal`.
    Dump,
}

/// Describe the action to take when there are conflicts. Not all
/// [`OnConflict`] are compatible with every [`Actions`]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum OnConflict {
    #[default]
    OverrideRemote,
    OverrideLocal,
    RenameRemote,
    RenameLocal,
    KeepLatest,
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
