pub mod backup;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum Actions {
    #[default]
    Backup,
    // Sync,
    // ZipBackup,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum OnConflict {
    OverrideRemote,
    OverrideLocal,
    RenameRemote,
    RenameLocal,

    #[default]
    Fail,
}
