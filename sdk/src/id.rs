use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FileID(pub i64);

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FolderID(pub i64);

impl FileID {
    pub fn id(&self) -> &i64 {
        let FileID(value) = self;
        value
    }
}

impl Display for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FileID(value) = self;
        write!(f, "fileid:{}", value)
    }
}

impl Display for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "folderid:{}", value)
    }
}

impl Debug for FileID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FileID(value) = self;
        write!(f, "fileid:{:?}", value)
    }
}

impl Debug for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "folderid:{:?}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fileid() {
        let fileid = FileID(23);
        assert_eq!(fileid.to_string(), "fileid:23");
        assert_eq!(format!("{fileid:?}"), "fileid:23");
    }

    #[test]
    fn test_folderid() {
        let folderid = FolderID(23);
        assert_eq!(folderid.to_string(), "folderid:23");
        assert_eq!(format!("{folderid:?}"), "folderid:23");
    }
}
