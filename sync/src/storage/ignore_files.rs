use std::path::{Path, PathBuf};

use crate::utils::locked_file::{LockedFile, ReadWrite};

use super::INSIDE_PROJECT_DIRECTORY;

const FILENAME: &str = ".pcloudignore";

pub struct IgnoreFilesContent {
    patterns: Vec<String>,
}

impl Default for IgnoreFilesContent {
    fn default() -> Self {
        Self {
            patterns: vec![
                INSIDE_PROJECT_DIRECTORY.to_string() + "/",
                ".git/".to_string(),
            ],
        }
    }
}

impl ReadWrite<IgnoreFilesContent> for IgnoreFilesContent {
    fn deserialize(content: &str) -> std::io::Result<IgnoreFilesContent> {
        Ok(IgnoreFilesContent {
            patterns: content.lines().map(|l| l.to_string()).collect(),
        })
    }

    fn serialize(object: &IgnoreFilesContent) -> std::io::Result<String> {
        Ok(object.patterns.join("\n") + "\n")
    }
}

pub type IgnoreFiles = LockedFile<IgnoreFilesContent>;

impl IgnoreFiles {
    pub fn path(project_dir: &Path) -> PathBuf {
        project_dir.join(FILENAME)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_path() {
        let base_path = Path::new("base");
        assert_eq!(
            IgnoreFiles::path(base_path),
            base_path.join(".pcloudignore")
        );
    }

    #[test]
    fn test_read() {
        let tmp_dir = tempdir().unwrap();
        let path = IgnoreFiles::path(tmp_dir.path());

        assert!(IgnoreFiles::read(&path).is_err());
        {
            let _lock = IgnoreFiles::update_or_create(&path).unwrap();
        }
        let ignored_files = IgnoreFiles::read(&path).unwrap();
        assert_eq!(ignored_files.content.patterns.len(), 2);

        let ignored_files2 = IgnoreFiles::read(&path).unwrap();
        assert_eq!(ignored_files2.content.patterns.len(), 2);

        assert_eq!(ignored_files.content.patterns, vec![".pcloud/", ".git/"]);
    }

    #[test]
    fn test_update_or_create() {
        let tmp_dir = tempdir().unwrap();
        let path = IgnoreFiles::path(tmp_dir.path());

        {
            let mut ignored_files = IgnoreFiles::update_or_create(&path).unwrap();
            ignored_files.content.patterns.push("ignore1".to_string());
            ignored_files.content.patterns.push("ignore2".to_string());
        }

        let ignored_files = IgnoreFiles::read(&path).unwrap();
        assert_eq!(ignored_files.content.patterns.len(), 4);
    }
}
