use fs4::FileExt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::ErrorKind;
use std::ops::Drop;
use std::path::{Path, PathBuf};
use tracing::debug;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILENAME: &str = "apps.json";

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FileData2 {
    pub apps: Vec<App>,
    version: String,
}

impl Default for FileData2 {
    fn default() -> Self {
        Self {
            apps: Vec::new(),
            version: VERSION.to_string(),
        }
    }
}

pub struct FileData {
    data: FileData2,
    path: PathBuf,
    file: File,
    write: bool,
}

impl FileData {
    pub fn apps(&self) -> &Vec<App> {
        &self.data.apps
    }
}

impl FileData {
    fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }

    fn ensure_exists(path: &Path) -> File {
        match File::open(path) {
            Ok(file) => file,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                std::fs::create_dir_all(path.parent().unwrap())
                    .expect("Cannot create home directory");
                confy::store_path(path, FileData2::default()).unwrap();
                File::open(path).unwrap()
            }
            Err(e) => panic!("Unhandled error: {e}"),
        }
    }

    pub fn read(home: &Path) -> FileData {
        let path = FileData::path(home);
        let file = FileData::ensure_exists(&path);

        debug!("Lock file (shared) '{}'", path.display());
        file.lock_shared().unwrap();

        debug!("Read file from '{}'", path.display());
        let cfg: FileData2 = confy::load_path(path.clone())
            .unwrap_or_else(|_| panic!("Failed to open '{}' file", path.display()));

        FileData {
            data: cfg,
            write: false,
            path,
            file,
        }
    }

    pub fn write(home: &Path) -> FileData {
        let path = FileData::path(home);
        let file = FileData::ensure_exists(&path);

        debug!("Lock file (exclusive) '{}'", path.display());
        file.lock_exclusive().unwrap();

        debug!("Read file from '{}'", path.display());
        let cfg: FileData2 = confy::load_path(&path)
            .unwrap_or_else(|_| panic!("Failed to open '{}' file", path.display()));

        FileData {
            data: cfg,
            write: true,
            path,
            file,
        }
    }
}

impl Drop for FileData {
    fn drop(&mut self) {
        if self.write {
            debug!("Save content to file '{}'", self.path.display());
            confy::store_path(&self.path, &self.data).unwrap();
        }
        debug!("Unlock file '{}'", self.path.display());
        self.file.unlock().unwrap();
    }
}
