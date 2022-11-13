use crate::utils::locked_file::{LockedFile, ReadWrite};
use std::io::{Read, Write};
use std::{fs::OpenOptions, path::Path};

const FILENAME: &str = ".pcloudignore";

#[derive(Default)]
pub struct IgnoreFilesContent {
    patterns: Vec<String>,
}

impl ReadWrite<IgnoreFilesContent> for IgnoreFilesContent {
    fn deserialize(content: &str) -> std::io::Result<IgnoreFilesContent> {
        Ok(IgnoreFilesContent {
            patterns: content.lines().map(|l| l.to_string()).collect(),
        })
    }

    fn serialize(object: &IgnoreFilesContent) -> std::io::Result<String> {
        Ok(object.patterns.join("\n"))
    }
}

pub type IgnoreFiles = LockedFile<IgnoreFilesContent>;



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_read() {
        let tmp_dir = tempdir().unwrap();

        let path = tmp_dir.path().join(FILENAME);
        let ignored_files = IgnoreFiles::read(&path);
        assert!(ignored_files.content.patterns.is_empty());

        let ignored_files2 = IgnoreFiles::read(&path);
        assert!(ignored_files2.content.patterns.is_empty());
    }

    #[test]
    fn test_write() {
        let tmp_dir = tempdir().unwrap();

        {
            let path = tmp_dir.path().join(FILENAME);
            let mut ignored_files = IgnoreFiles::write(&path);
            ignored_files.content.patterns.push("ignore1".to_string());
            ignored_files.content.patterns.push("ignore2".to_string());
        }

        let path = tmp_dir.path().join(FILENAME);
        let ignored_files = IgnoreFiles::read(&path);
        assert!(ignored_files.content.patterns.len() == 2);
    }
}
