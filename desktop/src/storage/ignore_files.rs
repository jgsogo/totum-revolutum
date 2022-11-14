use crate::utils::locked_file::{LockedFile, ReadWrite};

use std::path::{Path, PathBuf};

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
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_path() {
        let base_path = Path::new("base");
        assert!(IgnoreFiles::path(base_path) == base_path.join(".pcloudignore"));
    }

    #[test]
    fn test_read() {
        let tmp_dir = tempdir().unwrap();
        let path = IgnoreFiles::path(tmp_dir.path());

        let ignored_files = IgnoreFiles::read(&path);
        assert!(ignored_files.content.patterns.len() == 2);

        let ignored_files2 = IgnoreFiles::read(&path);
        assert!(ignored_files2.content.patterns.len() == 2);

        assert!(ignored_files.content.patterns == vec![".pcloud/", ".git/"]);
    }

    #[test]
    fn test_write() {
        let tmp_dir = tempdir().unwrap();
        let path = IgnoreFiles::path(tmp_dir.path());

        {
            let mut ignored_files = IgnoreFiles::write(&path);
            ignored_files.content.patterns.push("ignore1".to_string());
            ignored_files.content.patterns.push("ignore2".to_string());
        }

        let ignored_files = IgnoreFiles::read(&path);
        assert!(ignored_files.content.patterns.len() == 4);
    }
}
