use crate::locked_file::{LockedFile, LockedFileTrait};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const FILENAME: &str = ".pcloud";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Config {
    pub client_id: String,
    pub userid: i32,
}

impl Config {
    pub fn new(client_id: &str, userid: i32) -> Self {
        Self {
            client_id: client_id.to_string(),
            userid,
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
