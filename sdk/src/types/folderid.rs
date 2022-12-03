use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct FolderID(pub i64);

impl Display for FolderID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let FolderID(value) = self;
        write!(f, "folderid:{}", value)
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
    fn test_folderid() {
        let folderid = FolderID(23);
        assert_eq!(folderid.to_string(), "folderid:23");
        assert_eq!(format!("{folderid:?}"), "folderid:23");
    }
}
