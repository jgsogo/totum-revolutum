use std::collections::HashMap;
use utils::http::AddToParams;

use crate::methods::params::TargetLocation;
use crate::types::File;

pub struct SourceAndTargetFile {
    pub source: File,
    pub target: TargetLocation,
}

impl AddToParams for SourceAndTargetFile {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        self.source.add_to_params(params);
        self.target.add_to_params(params);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::methods::params::TargetLocation;
    use crate::types::{File, FileID, FolderID, RemotePath};

    use super::*;

    #[test]
    fn test_params_with_ids_noname() {
        let input = SourceAndTargetFile {
            source: FileID::new(1234).into(),
            target: TargetLocation::FolderAndName((FolderID::new(4321), None)),
        };
        let mut params = HashMap::new();
        input.add_to_params(&mut params);

        assert_eq!(params.len(), 2);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = SourceAndTargetFile {
            source: FileID::new(1234).into(),
            target: TargetLocation::FolderAndName((FolderID::new(4321), Some("name".to_string()))),
        };
        let mut params = HashMap::new();
        input.add_to_params(&mut params);

        assert_eq!(params.len(), 3);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_paths() {
        let input = SourceAndTargetFile {
            source: File::RemotePath(RemotePath::from_str("path:/from/path").unwrap()),
            target: TargetLocation::RemotePath(RemotePath::from_str("path:/to/path").unwrap()),
        };
        let mut params = HashMap::new();
        input.add_to_params(&mut params);

        assert_eq!(params.len(), 2);
        assert_eq!(params.get("path"), Some(&"/from/path".to_string()));
        assert_eq!(params.get("topath"), Some(&"/to/path".to_string()));
    }
}
