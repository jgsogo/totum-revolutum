use crate::locked_file::{LockedFile, LockedFileTrait};
use pcloud_sdk::data;
use std::path::{Path, PathBuf};

const FILENAME: &str = "apps.json";

pub struct AppsData {
    filedata: LockedFile<Vec<data::app::App>>,
}

impl AppsData {
    pub fn apps(&self) -> &Vec<data::app::App> {
        self.filedata.data()
    }

    pub fn apps_as_mut(&mut self) -> &mut Vec<data::app::App> {
        self.filedata.data_as_mut()
    }

    pub fn find(&self, client_id: &str) -> Result<&data::app::App, std::io::Error> {
        match self.apps().iter().find(|&app| app.client_id == client_id) {
            Some(app) => Ok(app),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Application not found with id '{}'", client_id),
            )),
        }
    }

    fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }
}

impl LockedFileTrait for AppsData {
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
