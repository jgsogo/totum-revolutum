use anyhow::Result;

use crate::methods::params::Params;
use crate::methods::params::ParamsType;
use crate::methods::params::TargetLocation;
use crate::types::Folder;

pub struct SourceAndTargetFolder {
    pub source: Folder,
    pub target: TargetLocation,
}

impl Params for SourceAndTargetFolder {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        self.source.add_to_params(params)?;
        self.target.add_to_params(params)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::methods::params::TargetLocation;
    use crate::types::FolderID;

    use super::*;

    #[test]
    fn test_params_with_ids_noname() {
        let input = SourceAndTargetFolder {
            source: Folder::FolderID(FolderID(1234)),
            target: TargetLocation::FolderAndName((FolderID(4321), None)),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = SourceAndTargetFolder {
            source: Folder::FolderID(FolderID(1234)),
            target: TargetLocation::FolderAndName((FolderID(4321), Some("name".to_string()))),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_paths() {
        let input = SourceAndTargetFolder {
            source: Folder::from_str("/from/path").unwrap(),
            target: TargetLocation::Path(PathBuf::from("/to/path")),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("path"), Some(&"/from/path".to_string()));
        assert_eq!(params.get("topath"), Some(&"/to/path".to_string()));
    }
}
