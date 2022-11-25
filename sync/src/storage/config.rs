use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::actions::{Actions, AfterSend, OnConflict};
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

    pub fn get_pcloud_client(&self, home: &Path) -> Result<pcloud_sdk::client::HttpClient> {
        // TODO: This is probably not the place for this function
        let apps_file_path = apps::AppsFile::path(home);
        let lock = apps::AppsFile::read(&apps_file_path);
        if let Ok(found) = lock.content.find(&self.client_id) {
            if let Ok(token) = found.find_token(self.userid) {
                return Ok(pcloud_sdk::client::HttpClient::new(token.clone()));
            }
        }
        Err(anyhow!("Cannot find pcloud client for the given config"))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ConfigAction {
    action: Actions,
    conflict: OnConflict,
    after_send: AfterSend,
    #[serde(with = "ts_seconds_option")]
    pub last_executed: Option<DateTime<Utc>>,
}

impl ConfigAction {
    pub fn action(&self) -> &Actions {
        &self.action
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub auth: ConfigAuth,
    pub action: ConfigAction,

    #[serde(skip)]
    pub pcloud: Option<pcloud_sdk::client::HttpClient>,
}

impl Config {
    pub fn new(client_id: &str, userid: i32, remote_path: Option<String>) -> Self {
        Self {
            auth: ConfigAuth::new(client_id, userid, remote_path),
            ..Default::default()
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

    pub fn read_with_pcloud_client(home: &Path, path: &Path) -> Result<ConfigFile> {
        let mut config = ConfigFile::read(path);
        config
            .content
            .data
            .auth
            .get_pcloud_client(home)
            .map(|pcloud| {
                config.content.data.pcloud = Some(pcloud);
                config
            })
    }

    pub fn write_with_pcloud_client(home: &Path, path: &Path) -> Result<ConfigFile> {
        let config = ConfigFile::write(path);
        match config {
            Ok(mut config) => config
                .content
                .data
                .auth
                .get_pcloud_client(home)
                .map(|pcloud| {
                    config.content.data.pcloud = Some(pcloud);
                    config
                }),
            Err(e) => Err(e),
        }
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
