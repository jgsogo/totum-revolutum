use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use pcloud_sdk::utils;

use crate::utils::{
    locked_file::{LockedFile, ReadWrite},
    versioned_data::VersionedData,
};
use crate::utils::{mut_find_or_insert, to_absolute_path};

const FILENAME: &str = "cron.yaml";

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Directory {
    path: String,
    cron: utils::cron::CronTz,
}

impl Directory {
    pub fn new(path: &Path, expression: &str, tz: &chrono_tz::Tz) -> Self {
        let path = to_absolute_path(path);

        Self {
            path: path
                .to_str()
                .expect("Cannot convert path to string")
                .to_string(),
            cron: utils::cron::CronTz::new(expression, tz),
        }
    }

    pub fn upcoming(&self) -> Option<chrono::DateTime<chrono_tz::Tz>> {
        self.cron.upcoming()
    }

    pub fn next<Tz>(&self, previous: &chrono::DateTime<Tz>) -> Option<chrono::DateTime<Tz>>
    where
        Tz: chrono::TimeZone,
    {
        self.cron.next(previous)
    }

    pub fn path(&self) -> PathBuf {
        Path::new(&self.path).to_path_buf()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Directories {
    pub directories: Vec<Directory>,
}

type DirectoriesContent = VersionedData<Directories>;

impl DirectoriesContent {
    pub fn find(&self, path: &Path) -> Option<&Directory> {
        self.data.directories.iter().find(|v| v.path() == path)
    }

    pub fn find_or_insert(&mut self, path: &Path, directory: Directory) -> (&mut Directory, bool) {
        mut_find_or_insert(&mut self.data.directories, |d| d.path() == path, directory)
    }
}

pub type DirectoriesFile = LockedFile<DirectoriesContent>;

impl DirectoriesFile {
    pub fn path(home: &Path) -> PathBuf {
        home.join(FILENAME)
    }
}

impl ReadWrite<DirectoriesContent> for DirectoriesContent {
    fn deserialize(content: &str) -> std::io::Result<DirectoriesContent> {
        Ok(serde_yaml::from_str(content).expect("cannot deserialize content"))
    }

    fn serialize(object: &DirectoriesContent) -> std::io::Result<String> {
        Ok(serde_yaml::to_string(&object).expect("Cannot serialize content"))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_path() {
        let base_path = Path::new("home");
        assert_eq!(
            DirectoriesFile::path(base_path),
            base_path.join("cron.yaml")
        );
    }

    #[test]
    fn test_read() {
        let tmp_dir = tempdir().unwrap();
        let path = DirectoriesFile::path(tmp_dir.path());

        assert!(DirectoriesFile::read(&path).is_err());
        {
            let _lock = DirectoriesFile::update_or_create(&path, Directories::default()).unwrap();
        }
        let directories_lock = DirectoriesFile::read(&path).unwrap();
        let directories = &directories_lock.content.data;
        assert!(directories.directories.is_empty());

        let directories_lock2 = DirectoriesFile::read(&path).unwrap();
        let directories2 = &directories_lock2.content.data;
        assert!(directories2.directories.is_empty());
    }

    #[test]
    fn test_update_or_create() {
        let tmp_dir = tempdir().unwrap();
        let path = DirectoriesFile::path(tmp_dir.path());

        {
            let mut directories_lock =
                DirectoriesFile::update_or_create(&path, Directories::default()).unwrap();
            let dirs = &mut directories_lock.content.data.directories;

            dirs.push(Directory::new(
                Path::new("path/to/dir"),
                "*/2 * * * *",
                &chrono_tz::Tz::from_str("UTC").unwrap(),
            ))
        }

        let directories_lock = DirectoriesFile::read(&path).unwrap();
        assert_eq!(directories_lock.content.data.directories.len(), 1);
    }
}
