use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::actions::{Actions, OnConflict};
use crate::remote::filesystem::PCloudHttpClient;
use crate::utils::locked_file::{LockedFile, ReadWrite};
use crate::utils::versioned_data::VersionedData;

use super::apps;
use super::INSIDE_PROJECT_DIRECTORY;

const FILENAME: &str = "config";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ConfigAuth {
    pub client_id: String,
    pub userid: i32,
    /// Path inside the remote application root path
    pub remote_path: Option<String>,
}

impl ConfigAuth {
    pub fn new(client_id: &str, userid: i32, remote_path: Option<String>) -> Self {
        Self {
            client_id: client_id.to_string(),
            userid,
            remote_path,
        }
    }

    pub fn get_pcloud_client(&self, home: &Path) -> Result<PCloudHttpClient> {
        // TODO: This is probably not the place for this function
        let apps_file_path = apps::AppsFile::path(home);
        let lock = apps::AppsFile::read(&apps_file_path)?;
        if let Ok(found) = lock.content.find(&self.client_id) {
            if let Ok(token) = found.find_token(self.userid) {
                return Ok(pcloud_sdk::client::HttpClient::new(token.clone()));
            }
        }
        Err(anyhow!("Cannot find pcloud client for the given config"))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ConfigAction {
    action: Actions,
    conflict: OnConflict,
    #[serde(with = "ts_seconds_option")]
    pub last_executed: Option<DateTime<Utc>>,
}

impl ConfigAction {
    /// Create a new `ConfigAction` instance. Not all [`OnConflict`] are compatible with
    /// all [`Actions`]
    pub fn new(action: Actions, conflict: OnConflict) -> Result<ConfigAction> {
        ConfigAction::check(&action, &conflict)?;

        Ok(ConfigAction {
            action,
            conflict,
            last_executed: None,
        })
    }

    /// Check if the given `action` and `conflict` are compatible
    ///
    /// * `Actions::Backup`: Send local content to remote. Local is never touched and nothing
    /// will be removed from remote. It can be combined with `OnConflict::OverrideRemote` or
    /// `OnConflict::RenameRemote`.
    ///
    /// * `Actions::ZipBackup`: Send local content to remote in a zip file. It can only be combined with
    /// `OnConflict::OverrideRemote` or `OnConflict::RenameRemote`
    ///
    /// * `Actions::Sync`: Keep local and remote synced. It can be combined with:
    ///    * `OnConflict::KeepLatest`: modification time will decide which file to keep
    ///    * `OnConflict::OverrideLocal`: local will always override remote
    ///    * `OnConflict::OverrideRemote`: remote will always override local
    ///
    /// * `Actions::Dump`: Send remote content to local folder. It can be combined with
    /// `OnConflict::OverrideLocal` or `OnConflict::RenameLocal`.
    ///
    /// * `Actions::MoveUpload`: Move local content to remote folder, local will be removed.
    /// It can be combined with `OnConflict::OverrideRemote` or `OnConflict::RenameRemote`.
    ///
    /// * `Actions::MoveDownload`: Move remote content to local folder, remote will be removed.
    /// It can be combined with `OnConflict::OverrideLocal` or `OnConflict::RenameLocal`.
    pub fn check(action: &Actions, conflict: &OnConflict) -> Result<()> {
        match (action, conflict) {
            (Actions::Backup, OnConflict::OverrideRemote) => Ok(()),
            (Actions::Backup, OnConflict::RenameRemote) => Ok(()),
            (Actions::ZipBackup, OnConflict::OverrideRemote) => Ok(()),
            (Actions::ZipBackup, OnConflict::RenameRemote) => Ok(()),
            (Actions::Sync, OnConflict::KeepLatest) => Ok(()),
            (Actions::Sync, OnConflict::OverrideLocal) => Ok(()),
            (Actions::Sync, OnConflict::OverrideRemote) => Ok(()),
            (Actions::Dump, OnConflict::OverrideLocal) => Ok(()),
            (Actions::Dump, OnConflict::RenameLocal) => Ok(()),
            (Actions::MoveUpload, OnConflict::OverrideRemote) => Ok(()),
            (Actions::MoveUpload, OnConflict::RenameRemote) => Ok(()),
            (Actions::MoveDownload, OnConflict::OverrideLocal) => Ok(()),
            (Actions::MoveDownload, OnConflict::RenameLocal) => Ok(()),
            _ => bail!("Invalid combination of action ({action:?}) and conflict resolution ({conflict:?})"),
        }
    }

    pub fn action(&self) -> &Actions {
        &self.action
    }

    pub fn conflict(&self) -> &OnConflict {
        &self.conflict
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub auth: ConfigAuth,
    pub action: ConfigAction,
}

impl Config {
    pub fn new(client_id: &str, userid: i32, remote_path: Option<String>, action: ConfigAction) -> Self {
        Self {
            auth: ConfigAuth::new(client_id, userid, remote_path),
            action,
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.auth == other.auth && self.action == other.action
    }
}

impl Eq for Config {}

pub type ConfigFileContent = VersionedData<Config>;
pub type ConfigFile = LockedFile<ConfigFileContent>;

impl ConfigFile {
    pub fn path(home: &Path) -> PathBuf {
        home.join(INSIDE_PROJECT_DIRECTORY).join(FILENAME)
    }
}

impl ReadWrite<ConfigFileContent> for ConfigFileContent {
    fn deserialize(content: &str) -> std::io::Result<ConfigFileContent> {
        Ok(serde_yaml::from_str(content).expect("cannot deserialize content"))
    }

    fn serialize(object: &ConfigFileContent) -> std::io::Result<String> {
        Ok(serde_yaml::to_string(&object).expect("Cannot serialize content"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let base_path = Path::new("base");
        assert!(ConfigFile::path(base_path) == base_path.join(".pcloud").join("config"));
    }
}
