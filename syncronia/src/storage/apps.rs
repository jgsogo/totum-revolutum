use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use pcloud_sdk::data;

use crate::utils::mut_find_or_insert;
use crate::utils::{
    locked_file::{LockedFile, ReadWrite},
    versioned_data::VersionedData,
};

const FILENAME: &str = "apps.yaml";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Apps {
    apps: Vec<data::app::App>,
}

type AppsFileContent = VersionedData<Apps>;

impl AppsFileContent {
    pub fn find(&self, client_id: &str) -> Result<&data::app::App, std::io::Error> {
        match self.data.apps.iter().find(|&app| app.client_id == client_id) {
            Some(app) => Ok(app),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Application not found with id '{client_id}'"),
            )),
        }
    }

    pub fn find_or_insert(&mut self, client_id: &str, app: data::app::App) -> (&mut data::app::App, bool) {
        mut_find_or_insert(&mut self.data.apps, |app| app.client_id == client_id, app)
    }
}

pub type AppsFile = LockedFile<AppsFileContent>;

impl AppsFile {
    pub fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }
}

impl ReadWrite<AppsFileContent> for AppsFileContent {
    fn deserialize(content: &str) -> std::io::Result<AppsFileContent> {
        Ok(serde_yaml::from_str(content).expect("cannot deserialize content"))
    }

    fn serialize(object: &AppsFileContent) -> std::io::Result<String> {
        Ok(serde_yaml::to_string(&object).expect("Cannot serialize content"))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_path() {
        let base_path = Path::new("home");
        assert_eq!(AppsFile::path(base_path), base_path.join("apps.yaml"));
    }

    #[test]
    fn test_read() {
        let tmp_dir = tempdir().unwrap();
        let path = AppsFile::path(tmp_dir.path());

        assert!(AppsFile::read(&path).is_err());
        {
            let _lock = AppsFile::update_or_create(&path, Apps::default()).unwrap();
        }
        let apps_lock = AppsFile::read(&path).unwrap();
        let apps = &apps_lock.content.data;
        assert!(apps.apps.is_empty());

        let apps_lock2 = AppsFile::read(&path).unwrap();
        let apps2 = &apps_lock2.content.data;
        assert!(apps2.apps.is_empty());
    }

    #[test]
    fn test_update_or_create() {
        let tmp_dir = tempdir().unwrap();
        let path = AppsFile::path(tmp_dir.path());
        {
            let mut apps_lock = AppsFile::update_or_create(&path, Apps::default()).unwrap();
            let apps = &mut apps_lock.content.data.apps;
            apps.push(data::app::App::new("name", "client_id", "client_secret"))
        }

        let apps_lock = AppsFile::read(&path).unwrap();
        assert_eq!(apps_lock.content.data.apps.len(), 1);
    }
}
