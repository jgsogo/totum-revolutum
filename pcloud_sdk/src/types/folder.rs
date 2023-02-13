use std::path::PathBuf;
use std::str::FromStr;

use crate::utils::normalize_path;

use super::FolderID;

#[derive(Debug, Clone)]
pub enum FolderValue {
    FolderID(FolderID),
    Path(PathBuf),
}

#[derive(Debug)]
pub struct Folder(pub FolderValue);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFolderError;

impl FromStr for Folder {
    type Err = ParseFolderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(f) = FolderID::from_str(s) {
            return Ok(Folder(FolderValue::FolderID(f)));
        } else if let Ok(p) = PathBuf::from_str(&s) {
            let p = normalize_path(p);
            if p.starts_with("/") && !p.starts_with("/..") {
                return Ok(Folder(FolderValue::Path(p)));
            }
        }
        Err(ParseFolderError)
    }
}
