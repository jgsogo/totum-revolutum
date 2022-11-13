use crate::locked_file::{LockedFile, LockedFileTrait};
use cron::Schedule;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use time::OffsetDateTime;

const FILENAME: &str = ".pcloud";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ConfigAuth {
    pub client_id: String,
    pub userid: i32,
}

impl ConfigAuth {
    pub fn new(client_id: &str, userid: i32) -> Self {
        Self {
            client_id: client_id.to_string(),
            userid,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum Actions {
    #[default]
    Backup,
    // Sync,
    // ZipBackup,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ConfigAction {
    pub action: Actions,
    pub cron: Option<String>,
    pub last_executed: Option<OffsetDateTime>,
}

impl ConfigAction {
    pub fn upcoming(&self) -> Option<OffsetDateTime> {
        match &self.cron {
            None => None,
            Some(expression) => {
                let schedule = Schedule::from_str(expression).unwrap();
                let next_time = schedule.upcoming(chrono::Utc).take(1).next();
                next_time.map(|t| OffsetDateTime::from_unix_timestamp(t.timestamp()).unwrap())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    pub auth: ConfigAuth,
    pub action: ConfigAction,
    pub ignore: Vec<String>,
}

impl Config {
    pub fn new(client_id: &str, userid: i32) -> Self {
        Self {
            auth: ConfigAuth::new(client_id, userid),
            ignore: vec![FILENAME.to_string(), ".git".to_string()],
            ..Default::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auth: ConfigAuth::default(),
            action: ConfigAction::default(),
            ignore: vec![FILENAME.to_string(), ".git".to_string()],
        }
    }
}

pub struct ConfigData {
    filedata: LockedFile<Config>,
}

impl ConfigData {
    pub fn config(&self) -> &Config {
        self.filedata.data()
    }

    pub fn config_as_mut(&mut self) -> &mut Config {
        self.filedata.data_as_mut()
    }

    fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }
}

impl LockedFileTrait for ConfigData {
    fn read(home: &Path) -> Self {
        let path = ConfigData::path(home);
        ConfigData {
            filedata: LockedFile::read(&path),
        }
    }

    fn write(home: &Path) -> Self {
        let path = ConfigData::path(home);
        ConfigData {
            filedata: LockedFile::write(&path),
        }
    }
}
