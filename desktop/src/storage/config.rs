use crate::locked_file::{LockedFile, LockedFileTrait};

use chrono::serde::ts_seconds_option;
use chrono::{DateTime, TimeZone, Utc};

use cron_parser::parse;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const FILENAME: &str = ".pcloud/config";

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CronTz {
    pub expression: String,
    tz: String,
}

impl CronTz {
    pub fn cron_tz(&self) -> chrono_tz::Tz {
        self.tz.parse().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct ConfigAction {
    pub action: Actions,
    pub cron: Option<CronTz>,
    #[serde(with = "ts_seconds_option")]
    pub last_executed: Option<DateTime<Utc>>,
}

impl ConfigAction {
    pub fn upcoming(&self) -> Option<DateTime<chrono_tz::Tz>> {
        match &self.cron {
            None => None,
            Some(cron) => {
                let tz = cron.cron_tz();

                let now_tz = tz
                    .from_local_datetime(&chrono::Utc::now().naive_utc())
                    .unwrap();

                let schedule = parse(&cron.expression, &now_tz).unwrap();
                Some(schedule)
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
