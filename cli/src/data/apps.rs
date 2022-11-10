use fs4::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::ops::Drop;
use std::path::{Path, PathBuf};
use tracing::debug;

const FILENAME: &str = "apps.json";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Apps {
    apps: Vec<Apps>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct App {
    client_id: String,
    client_secret: String,

    tokens: Vec<OAuth2Token>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct OAuth2Token {
    pub userid: i32,
    pub locationid: u8,
    pub access_token: String,
    pub token_type: String,
}

pub struct FileData {
    pub content: Apps,
    file: Option<File>,
}

impl FileData {
    fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }

    pub fn read(home: &Path) -> FileData {
        debug!("Read file from {}", home.display());
        let path = FileData::path(home);

        match File::open(path.clone()) {
            Ok(mut file) => {
                debug!("Lock file before reading");
                file.lock_shared().unwrap();
                debug!("Read JSON from file");
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                FileData {
                    content: serde_json::from_str(&data).expect("JSON was not well-formatted"),
                    file: Some(file),
                }
            }
            Err(e) => {
                debug!("Cannot read the file {}", e);
                if path.exists() {
                    panic!("Cannot open file: {}", e);
                }
                FileData {
                    content: Apps::default(),
                    file: None,
                }
            }
        }
    }
}

impl Drop for FileData {
    fn drop(&mut self) {
        debug!("Drop file");
        if let Some(file) = self.file.take() {
            file.unlock().unwrap();
        }
    }
}
