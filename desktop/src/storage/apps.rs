use crate::locked_file::{LockedFile, LockedFileTrait};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILENAME: &str = "apps.json";

// TODO: Move these structs to the SDK

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,

    pub tokens: Vec<OAuth2Token>,
}

impl App {
    pub fn new(name: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            name: name.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn default(client_id: &str, client_secret: &str) -> Self {
        Self {
            name: "no-name".to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            tokens: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct OAuth2Token {
    pub userid: i32,
    pub locationid: u8,
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FileData2 {
    version: String,
    pub apps: Vec<App>,
}

impl Default for FileData2 {
    fn default() -> Self {
        Self {
            apps: Vec::new(),
            version: VERSION.to_string(),
        }
    }
}

// pub type FileData = LockedFile<FileData2>;

pub struct AppsData {
    filedata: LockedFile<FileData2>,
}

impl AppsData {
    pub fn apps(&self) -> &Vec<App> {
        &self.filedata.data.apps
    }

    pub fn apps_as_mut(&mut self) -> &mut Vec<App> {
        assert!(self.writable(), "Write is required to borrow mut");
        &mut self.filedata.data.apps
    }

    fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }
}

impl LockedFileTrait for AppsData {
    fn writable(&self) -> bool {
        self.filedata.writable()
    }

    fn read(home: &Path) -> Self {
        let path = AppsData::path(home);
        AppsData {
            filedata: LockedFile::read(&path),
        }
    }

    fn write(home: &Path) -> Self {
        let path = AppsData::path(home);
        AppsData {
            filedata: LockedFile::write(&path),
        }
    }
}
